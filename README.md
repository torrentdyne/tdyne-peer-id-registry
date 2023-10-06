[![Build Status](https://github.com/torrentdyne/tdyne-peer-id-registry/actions/workflows/ci.yml/badge.svg)](https://github.com/torrentdyne/tdyne-peer-id-registry/actions/workflows/ci.yml)

<!-- cargo-rdme start -->

# BitTorrent peer ID registry/parser/(soon) encoder

By convention, BitTorrent clients identify themselves and their versions in
peer IDs they send to trackers and other clients.
Unfortunately, there is no single client/version encoding, so over time different clients
adopted different conventions, which made parsing peer IDs difficult.
This crate provides a comprehensive peer ID parser and a registry of all known
BitTorrent clients.

Peer ID encoding is one of immediate goals, see "Roadmap"
below for details.

This crate uses [`tdyne_peer_id`] to encode peer IDs.

Example:

```rust
use tdyne_peer_id::PeerId;
use tdyne_peer_id_registry::parse;

let peer_id = PeerId::from(b"-TR404Z-*\x00\x01d7xkqq04n");

let parsed = parse(peer_id).expect("recognised peer ID");
assert_eq!(parsed.client, "Transmission");

let version = parsed.version
    .expect("valid version encoding")
    .expect("Transmission does encode a version in its peer ID");
assert_eq!(version, "4.0.4 (Dev)");
```

## Current status

* used in production on [TORRENTDYNE](https://torrentdyne.com)
* test parity with Webtorrent's [`bittorrent-peerid`](https://github.com/webtorrent/bittorrent-peerid),
  the most popular JS implementation of BitTorrent peer ID parsing
* improves on [`bittorrent-peerid`](https://github.com/webtorrent/bittorrent-peerid) by
  recognising more clients and versions
* regularly fuzzed to verify absence of panics

## Roadmap

### Encoding peer IDs

A peer ID formatting API that only accepts known clients (in release mode) and
takes choices out of peer ID formatting would help the ecosystem to stay more consistent.

### Test parity with Transmission

Transmission has
[an extensive peer ID parser](https://github.com/transmission/transmission/blob/0c52b710ad241c2b68cb9c7a9eb68a8532b290d0/libtransmission/clients.cc).
Right now the Venn diagram of clients that `tdyne_peer_id_registry` and Transmission
can handle is two intersecting circles. It needs to get closer to two concentric rings.

### Structured allocation-free API

`tdyne_peer_id_registry` is designed to parse into an allocation-free tree
of structs and enums. In the current version the tree is not exposed as it is not stable yet
and changing it would be a breaking change. Instead, the API immediately turns the tree
into strings, the lowest common denominator. However, exposing the tree directly
would help projects that need to act on the information from the peer ID,
as they would be able to work directly with the structures instead of re-parsing strings.

<!-- cargo-rdme end -->

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
