use pretty_assertions::assert_eq;
use tdyne_peer_id::PeerId;
use test_case::test_case;

use tdyne_peer_id_registry::parse;

#[test_case(b"-BI3401-Em6o1EmvwLtD", "BiglyBT", "3.4.0.1")]
#[test_case(b"-BI5701-Axxxxxxxxxxx", "BiglyBT for Android", "5.7.0.1")]
fn biglybt_tests(peer_id_bytes: &[u8], client_name: &str, test_version: &str) {
    let peer_id = PeerId::try_from(peer_id_bytes).unwrap();
    let parsed = parse(peer_id).unwrap();
    assert_eq!(parsed.client, client_name);
    assert_eq!(&parsed.version.unwrap().unwrap(), test_version);
}
