// RGB C bindings library (librgb)
// Written in 2020 by
//     Alekos Filini,
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>,
//     Zoe Faltibà <zoefaltiba@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_uchar};
use std::str::FromStr;
use std::iter::FromIterator;

#[cfg(not(target_os = "android"))]
use log::LevelFilter;
#[cfg(not(target_os = "android"))]
use std::env;

use bitcoin::OutPoint;
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::blockdata::transaction::ParseOutPointError;
use bitcoin::consensus::{Decodable, Encodable};

use lnpbp::Chain;
use lnpbp::seals::{OutpointReveal, OutpointHash};
use rgb::{
    Consignment, ContractId, FromBech32, Genesis, //SealDefinition,
    SealEndpoint, AtomicValue,
};

use rgb_node::rpc::reply::SyncFormat;
use rgb20::{Asset, Invoice, Outpoint, OutpointCoins, SealCoins};
use rgb_node::i9n::{Config, Runtime};
use lnpbp::strict_encoding::strict_deserialize;
use lnpbp::bech32::ToBech32String;
use rgb_node::rgbd::ContractName;
use rgb_node::util::file::ReadWrite;
use microservices::FileFormat;
use internet2::ZmqSocketAddr;

use crate::error::RequestError;
use crate::helpers::*;
use lnpbp::client_side_validation::CommitConceal;

pub(crate) fn _connect_rgb(
    datadir: *const c_char,
    network: *const c_char,
    stash_rpc_endpoint: *const c_char,
    contract_endpoints: *const c_char,
    electrum: *const c_char,
    verbosity: u8,
) -> Result<Runtime, RequestError> {
    let c_network = unsafe { CStr::from_ptr(network) };
    let network = Chain::from_str(c_network.to_str()?)?;

    let c_stash_rpc_endpoint = unsafe { CStr::from_ptr(stash_rpc_endpoint) };
    let stash_rpc_endpoint = ZmqSocketAddr::from_str(c_stash_rpc_endpoint.to_str()?)?;

    let c_electrum = unsafe { CStr::from_ptr(electrum) };
    let electrum = c_electrum.to_str()?.to_string();

    let s_contract_endpoints: HashMap<ContractName, String> =
        serde_json::from_str(&ptr_to_string(contract_endpoints)?)?;

    let mut contract_endpoints= map!{};
    for (name, url) in s_contract_endpoints {
        contract_endpoints.insert(name, ZmqSocketAddr::from_str(url.as_str())?);
    }

    let c_datadir = unsafe { CStr::from_ptr(datadir) };
    let datadir = c_datadir.to_str()?.to_string();

    let config = Config {
        network,
        stash_rpc_endpoint,
        data_dir: datadir,
        contract_endpoints,
        electrum_server: electrum,
        run_embedded: false,
        verbose: verbosity,
        ..Default::default()
    };

    info!("{:?}", config);

    let runtime = Runtime::init(config)?;

    Ok(runtime)
}

pub(crate) fn _run_rgb_embedded(
    datadir: *const c_char,
    network: *const c_char,
    electrum: *const c_char,
    verbosity: u8,
) -> Result<Runtime, RequestError> {
    let c_network = unsafe { CStr::from_ptr(network) };
    let network = Chain::from_str(c_network.to_str()?)?;

    let c_datadir = unsafe { CStr::from_ptr(datadir) };
    let datadir = c_datadir.to_str()?.to_string();

    let c_electrum = unsafe { CStr::from_ptr(electrum) };
    let electrum = c_electrum.to_str()?.to_string();

    let contract_endpoints: HashMap<ContractName, ZmqSocketAddr> =
        [(ContractName::Fungible, ZmqSocketAddr::from_str("inproc://fungible-rpc")?)]
            .iter()
            .cloned()
            .collect();
    let stash_rpc_endpoint = ZmqSocketAddr::from_str("inproc://stash-rpc")?;

    let config = Config {
        network,
        data_dir: datadir,
        stash_rpc_endpoint,
        contract_endpoints,
        electrum_server: electrum,
        run_embedded: true,
        verbose: verbosity,
    };

    info!("{:?}", config);

    let runtime = Runtime::init(config)?;

    Ok(runtime)
}

#[cfg(target_os = "android")]
pub(crate) fn _start_logger() {
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Debug),
    );
}

#[cfg(not(target_os = "android"))]
pub(crate) fn _start_logger() {
    env::set_var("RUST_LOG", "trace");
    ::env_logger::init();
    log::set_max_level(LevelFilter::Trace);
}

pub(crate) fn _issue(
    runtime: &COpaqueStruct,
    network: *const c_char,
    ticker: *const c_char,
    name: *const c_char,
    description: *const c_char,
    precision: c_uchar,
    allocations: *const c_char,
    inflation: *const c_char,
    renomination: *const c_char,
    epoch: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let network = Chain::from_str(&ptr_to_string(network)?)?;

    let ticker = ptr_to_string(ticker)?;

    let name = ptr_to_string(name)?;

    let description = if description.is_null() {
        None
    } else {
        let description = ptr_to_string(description)?;
        if description.is_empty() {
            None
        } else {
            Some(description)
        }
    };

    let allocations: Vec<OutpointCoins> =
        serde_json::from_str(&ptr_to_string(allocations)?)?;

    let inflation: Vec<OutpointCoins> =
        serde_json::from_str(&ptr_to_string(inflation)?)?;

    let renomination: Option<OutPoint> =
        serde_json::from_str(&ptr_to_string(renomination)?)?;

    let epoch: Option<OutPoint> = serde_json::from_str(&ptr_to_string(epoch)?)?;

    debug!(
        "Issue: {{ network: {}, ticker: {}, name: {}, description: {:?}, \
        precision: {}, allocations: {:?}, inflation: {:?}, renomination: {:?}, \
        epoch: {:?} }}",
        network,
        ticker,
        name,
        description,
        precision,
        allocations,
        inflation,
        renomination,
        epoch
    );

    let asset = runtime.issue(
        network,
        ticker,
        name,
        description,
        precision,
        allocations,
        inflation,
        renomination,
        epoch,
    )?;

    Ok(serde_json::to_string(&asset)?)
}

pub(crate) fn _list_assets(
    runtime: &COpaqueStruct,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let SyncFormat(_, data) = runtime.list_assets(FileFormat::StrictEncode)?;
    let assets: Vec<Asset> = strict_deserialize(&data)?;

    let json_response = serde_json::to_string(&assets)?;
    Ok(json_response)
}

pub(crate) fn _asset_allocations(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let c_contract_id = unsafe { CStr::from_ptr(contract_id) };
    let contract_id = ContractId::from_bech32_str(c_contract_id.to_str()?)?;

    debug!("AssetAllocationsArgs {{ contract_id: {} }}", contract_id);

    let response = runtime.asset_allocations(contract_id)?;
    let json_response = serde_json::to_string(&response)?;
    Ok(json_response)
}

pub(crate) fn _outpoint_assets(
    runtime: &COpaqueStruct,
    outpoint: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let c_outpoint = unsafe { CStr::from_ptr(outpoint) };
    let outpoint = OutPoint::from_str(c_outpoint.to_str()?)?;

    debug!("Listing assets for {}", outpoint);

    let response = runtime.outpoint_assets(outpoint)?;
    let json_response = serde_json::to_string(&response)?;
    Ok(json_response)
}

pub(crate) fn _export_asset(
    runtime: &COpaqueStruct,
    asset_id: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let asset_id = ContractId::from_str(&ptr_to_string(asset_id)?)?;

    debug!("Exporting asset: {}", asset_id);

    let genesis = runtime.export_asset(asset_id)?;
    Ok(genesis.to_string())
}

pub(crate) fn _import_asset(
    runtime: &COpaqueStruct,
    asset_genesis: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let asset_genesis =
        Genesis::from_bech32_str(&ptr_to_string(asset_genesis)?)?;

    debug!("Importing asset: {}", asset_genesis);

    runtime.import_asset(asset_genesis)?;

    Ok(())
}

pub(crate) fn _invoice(
    asset_id: *const c_char,
    amount: c_double,
    outpoint: *const c_char,
) -> Result<String, RequestError> {
    let contract_id = ContractId::from_str(&ptr_to_string(asset_id)?)?;

    let outpoint = OutPoint::from_str(&ptr_to_string(outpoint)?)?;

    let outpoint_reveal = OutpointReveal::from(outpoint);
    let invoice = Invoice {
        contract_id,
        outpoint: Outpoint::BlindedUtxo(outpoint_reveal.commit_conceal()),
        amount,
    };

    debug!(
        "Created invoice {}, blinding factor {}",
        invoice, outpoint_reveal.blinding
    );

    let json_response = json!({
        "invoice": invoice.to_string(),
        "secret": outpoint_reveal.blinding
    });
    Ok(json_response.to_string())
}

pub(crate) fn _transfer(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
    inputs: *const c_char,
    payment: *const c_char,
    change: *const c_char,
    witness: *const c_char,
) -> Result<String, RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let contract_id = ContractId::from_str(&ptr_to_string(contract_id)?)?;

    let v_inputs: Vec<OutPoint> = serde_json::from_str(&ptr_to_string(inputs)?)?;
    let inputs: BTreeSet<OutPoint> = BTreeSet::from_iter(v_inputs.into_iter());

    let v_payments: Vec<String> = serde_json::from_str(&ptr_to_string(payment)?)?;
    let mut payment: BTreeMap<SealEndpoint, AtomicValue> = bmap!{};
    for payment_item in v_payments {
        let parts: Vec<&str> = payment_item.split('@').collect();

        if parts.len() == 2 {
            let hash = OutpointHash::from_str(parts[0])?;
            payment.insert(hash.into(), parts[1].parse()?);
        }
        else {
            return Err(RequestError::Input(s!("Invalid payment format; expected '<value>@<hash>'")));
        }
    }

    let v_changes: Vec<String> = serde_json::from_str(&ptr_to_string(change)?)?;
    let mut change = bmap!{};
    for change_item in v_changes {
        let seal_coins = SealCoins::from_str(&change_item)
            .map_err(|_| {
                RequestError::Outpoint(ParseOutPointError::Format)
            })?;

        change.insert(seal_coins.seal_definition(), seal_coins.coins);
    }

    let c_witness = unsafe { CStr::from_ptr(witness) };
    let mut data = c_witness.to_bytes();
    let witness = PartiallySignedTransaction::consensus_decode(&mut data)?;

    debug!(
        "TransferArgs {{contract_id: {}, inputs: {:?}, payment: {:?}, change: {:?}, witness: {:?}}}",
        contract_id, inputs, payment, change, witness,
    );

    let transfer = runtime.transfer(
        contract_id,
        inputs,
        payment,
        change,
        witness,
    )?;

    let mut data = vec![];
    transfer.witness.consensus_encode(&mut data)?;

    let json_transfer = json!({
        "consignment": transfer.consignment.to_bech32_string(),
        "witness": String::from_utf8(data)
            .map_err(|e| e.utf8_error())?
    });
    Ok(json_transfer.to_string())
}

pub(crate) fn _validate(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let filename = ptr_to_string(consignment_file)?;
    debug!("Reading consignment from {}", filename);
    let consignment = Consignment::read_file(filename.into())?;

    trace!("ValidateArgs {{ consignment: {:?} }}", consignment);

    runtime.validate(consignment)?;

    Ok(())
}

pub(crate) fn _accept(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
    reveal_outpoints: *const c_char,
) -> Result<(), RequestError> {
    let runtime = Runtime::from_opaque(runtime)?;

    let filename = ptr_to_string(consignment_file)?;
    debug!("Reading consignment from {}", filename);
    let consignment = Consignment::read_file(filename.into())?;

    let reveal_outpoints: Vec<OutpointReveal> =
        serde_json::from_str(&ptr_to_string(reveal_outpoints)?)?;

    trace!(
        "AcceptArgs {{ consignment: {:?}, reveal_outpoints: {:?} }}",
        consignment,
        reveal_outpoints
    );

    runtime.accept(consignment, reveal_outpoints)?;

    Ok(())
}
