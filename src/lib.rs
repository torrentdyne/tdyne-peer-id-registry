#![warn(missing_docs)]

//! # BitTorrent peer ID registry/parser/(soon) encoder
//!
//! By convention, BitTorrent clients identify themselves and their versions in
//! peer IDs they send to trackers and other clients.
//! Unfortunately, there is no single client/version encoding, so over time different clients
//! adopted different conventions, which made parsing peer IDs difficult.
//! This crate provides a comprehensive peer ID parser and a registry of all known
//! BitTorrent clients.
//!
//! Peer ID encoding is one of immediate goals, see "Roadmap"
//! below for details.
//!
//! This crate uses [`tdyne_peer_id`] to encode peer IDs.
//!
//! Example:
//!
//! ```
//! use tdyne_peer_id::PeerId;
//! use tdyne_peer_id_registry::parse;
//!
//! let peer_id = PeerId::from(b"-TR404Z-*\x00\x01d7xkqq04n");
//!
//! let parsed = parse(peer_id).expect("recognised peer ID");
//! assert_eq!(parsed.client, "Transmission");
//!
//! let version = parsed.version
//!     .expect("valid version encoding")
//!     .expect("Transmission does encode a version in its peer ID");
//! assert_eq!(version, "4.0.4 (Dev)");
//! ```
//!
//! ## Current status
//!
//! * used in production on [TORRENTDYNE](https://torrentdyne.com)
//! * test parity with Webtorrent's [`bittorrent-peerid`](https://github.com/webtorrent/bittorrent-peerid),
//!   the most popular JS implementation of BitTorrent peer ID parsing
//! * improves on [`bittorrent-peerid`](https://github.com/webtorrent/bittorrent-peerid) by
//!   recognising more clients and versions
//! * regularly fuzzed to verify absence of panics
//!
//! ## Roadmap
//!
//! ### Encoding peer IDs
//!
//! A peer ID formatting API that only accepts known clients (in release mode) and
//! takes choices out of peer ID formatting would help the ecosystem to stay more consistent.
//!
//! ### Test parity with Transmission
//!
//! Transmission has
//! [an extensive peer ID parser](https://github.com/transmission/transmission/blob/0c52b710ad241c2b68cb9c7a9eb68a8532b290d0/libtransmission/clients.cc).
//! Right now the Venn diagram of clients that `tdyne_peer_id_registry` and Transmission
//! can handle is two intersecting circles. It needs to get closer to two concentric rings.
//!
//! ### Structured allocation-free API
//!
//! `tdyne_peer_id_registry` is designed to parse into an allocation-free tree
//! of structs and enums. In the current version the tree is not exposed as it is not stable yet
//! and changing it would be a breaking change. Instead, the API immediately turns the tree
//! into strings, the lowest common denominator. However, exposing the tree directly
//! would help projects that need to act on the information from the peer ID,
//! as they would be able to work directly with the structures instead of re-parsing strings.

use crate::client::Client;
use crate::errors::{ClientParsingError, VersionParsingError};
use tdyne_peer_id::PeerId;

mod client;
mod client_styles;
///
pub mod errors;
mod known_clients;
mod version;
mod version_utils;

/// Human-readable representation of the client and the version (if it exists) encoded
/// in the parsed peer ID.
#[derive(Debug, Clone)]
pub struct Parsed {
    /// Name of the client. Can include suffixes such as `(Dev)` or `(Beta)`.
    pub client: String,
    /// Version, if any. The outer `Result` encodes parsing errors, while the internal
    /// `Option` can be `None` if the recognised client doesn't encode a version.
    /// Can also be set to `[unknown version]` if the version exists, but is not parsed yet.
    /// If you see this value, please consider reporting the corresponding peer ID
    /// in a github issue.
    pub version: Result<Option<String>, VersionParsingError>,
}

/// The main entry point for the library. Returns [`Parsed`] with human-readable string
/// representation of the parsed client and version. See [`Parsed`] for more details.
///
/// Example:
///
/// ```
/// use tdyne_peer_id::PeerId;
/// use tdyne_peer_id_registry::{parse, Parsed};
///
/// let parsed = parse(PeerId::from(b"-TR4040-xxxxxxxxxxxx"));
/// assert_eq!(
///     format!("{parsed:?}").as_str(),
///     r#"Ok(Parsed { client: "Transmission", version: Ok(Some("4.0.4")) })"#
/// );
/// ```
pub fn parse(peer_id: PeerId) -> Result<Parsed, ClientParsingError> {
    let client = Client::try_from(peer_id)?;

    Ok(Parsed {
        client: client.to_canonical().to_string(),
        version: parse_version(client, peer_id),
    })
}

fn parse_version(client: Client, peer_id: PeerId) -> Result<Option<String>, VersionParsingError> {
    Ok(client.parse_version(peer_id)?.map(|x| x.to_string()))
}
