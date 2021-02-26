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

#[derive(Debug, Display, From, Error)]
#[display(doc_comments)]
#[non_exhaustive]
pub(crate) enum RequestError {
    /// Bech32 error: {_0}
    #[from]
    Bech32(rgb::bech32::Error),

    /// Input value is not a JSON object or JSON parse error: {_0}
    #[from]
    Json(serde_json::Error),

    /// Invoice error: {_0}
    #[from]
    Invoice(rgb20::InvoiceError),

    /// Input value is not a UTF8 string: {_0}
    #[from]
    Utf8(std::str::Utf8Error),

    /// Invalid network/chain identifier: {_0}
    #[from]
    ChainParse(lnpbp::chain::ParseError),

    /// Bootstrap error: {_0}
    #[from]
    Runtime(rgb_node::error::BootstrapError),

    /// Transport error: {_0}
    #[from]
    Transport(internet2::transport::Error),

    /// Integration error: {_0}
    #[from]
    Integration(rgb_node::i9n::Error),

    /// Impossible error: {_0}
    #[from]
    Infallible(std::convert::Infallible),

    /// Outpoint parsing error: {_0}
    #[from]
    Outpoint(bitcoin::blockdata::transaction::ParseOutPointError),

    /// I/O error: {_0}
    #[from]
    Io(std::io::Error),

    /// Input error: {_0}
    #[from]
    Input(String),

    /// Strict encoding error: {_0}
    #[from]
    StrictEncoding(lnpbp::strict_encoding::Error),

    /// ZeroMQ socket address error: {_0}
    #[from]
    ZmqSocketAddr(internet2::AddrError),

    /// Hex format error: {_0}
    #[from]
    Hex(bitcoin::hashes::hex::Error),

    /// Parse integer error: {_0}
    #[from]
    ParseInt(std::num::ParseIntError),

    /// Bitcoin consensus encode error: {_0}
    #[from]
    ConsensusEncode(bitcoin::consensus::encode::Error),
}
