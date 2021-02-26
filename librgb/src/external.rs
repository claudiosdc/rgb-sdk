// RGB C bindings library (librgb)
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::os::raw::{c_char, c_double, c_uchar};

use crate::helpers::*;
use crate::internal::*;

#[no_mangle]
pub extern "C" fn rgb_node_connect(
    datadir: *const c_char,
    network: *const c_char,
    stash_rpc_endpoint: *const c_char,
    contract_endpoints: *const c_char,
    electrum: *const c_char,
    verbosity: c_uchar,
) -> CResult {
    _start_logger();

    info!("Connecting RGB node...");

    _connect_rgb(
        datadir,
        network,
        stash_rpc_endpoint,
        contract_endpoints,
        electrum,
        verbosity,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn rgb_node_run(
    datadir: *const c_char,
    network: *const c_char,
    electrum: *const c_char,
    verbosity: c_uchar,
) -> CResult {
    _start_logger();

    info!("Running embedded RGB node...");

    _run_rgb_embedded(datadir, network, electrum, verbosity).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_issue(
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
) -> CResultString {
    _issue(
        runtime,
        network,
        ticker,
        name,
        description,
        precision,
        allocations,
        inflation,
        renomination,
        epoch,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_list_assets(
    runtime: &COpaqueStruct,
) -> CResultString {
    _list_assets(runtime).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_asset_allocations(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
) -> CResultString {
    _asset_allocations(runtime, contract_id).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_outpoint_assets(
    runtime: &COpaqueStruct,
    outpoint: *const c_char,
) -> CResultString {
    _outpoint_assets(runtime, outpoint).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_export_asset(
    runtime: &COpaqueStruct,
    asset_id: *const c_char,
) -> CResultString {
    _export_asset(runtime, asset_id).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_import_asset(
    runtime: &COpaqueStruct,
    asset_genesis: *const c_char,
) -> CResult {
    _import_asset(runtime, asset_genesis).into()
}

#[no_mangle]
pub extern "C" fn rgb20_invoice(
    asset_id: *const c_char,
    amount: c_double,
    outpoint: *const c_char,
) -> CResultString {
    _invoice(asset_id, amount, outpoint).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_transfer(
    runtime: &COpaqueStruct,
    contract_id: *const c_char,
    inputs: *const c_char,
    payment: *const c_char,
    change: *const c_char,
    witness: *const c_char,
) -> CResultString {
    _transfer(
        runtime,
        contract_id,
        inputs,
        payment,
        change,
        witness,
    )
    .into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_validate(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
) -> CResult {
    _validate(runtime, consignment_file).into()
}

#[no_mangle]
pub extern "C" fn rgb_node_fungible_accept(
    runtime: &COpaqueStruct,
    consignment_file: *const c_char,
    reveal_outpoints: *const c_char,
) -> CResult {
    _accept(runtime, consignment_file, reveal_outpoints).into()
}
