#![no_main]

use libfuzzer_sys::fuzz_target;

use tdyne_peer_id::PeerId;
use tdyne_peer_id_registry::parse;

fuzz_target!(|data: [u8; 20]| {
    let peer_id = PeerId::from(data);
    let _ = parse(peer_id);
});
