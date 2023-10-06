use pretty_assertions::assert_eq;
use tdyne_peer_id::PeerId;
use tdyne_peer_id_registry::errors::ClientParsingError;
use tdyne_peer_id_registry::parse;
use test_case::test_case;

pub fn raw_or_hex(bytes: &[u8]) -> Vec<u8> {
    match bytes.len() {
        20 => bytes.to_vec(),
        40 => {
            let s = String::from_utf8_lossy(bytes).to_string();
            (0..40)
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..=(i + 1)], 16).unwrap())
                .collect()
        }
        _ => unreachable!("the argument must be 20 or 40 long"),
    }
}

// Inspired by cases covered in
// https://github.com/webtorrent/bittorrent-peerid/blob/f8457f24ef95b3e5eaa134bf0b5e264580c0eb09/test/basic.js

// added manually extracted version numbers to some of the tests
//
// "basic clients…" are all duplicated in "Azureus-style clients"
//
// "Azureus-style clients"
//
#[test_case(b"-AG2053-Em6o1EmvwLtD", "Ares", None)]
#[test_case(b"-AR1670-3Ql6wM3hgtCc", "Ares", None)]
#[test_case(b"-AT2520-vEEt0wO6v0cr", "Artemis", None)]
#[test_case(b"-AZ2200-6wfG2wk6wWLc", "Vuze", None)]
#[test_case(b"-NE1090002IKyMn4g7Ko", "BT Next Evolution", None)]
#[test_case(b"-BR0332-!XVceSn(*KIl", "BitRocket", None)]
#[test_case(b"2D46473031383075F80057821359D64BB3DFD265", "FlashGet", None)]
#[test_case(b"-GR6300-13s3iFKmbArc", "GetRight", None)]
#[test_case(b"-HL0290-xUO*9ugvENUE", "Halite", None)]
#[test_case(b"-KT11R1-693649213030", "KTorrent", None)]
#[test_case(b"2D4B543330302D006A7139727958377731756A4B", "KTorrent", None)]
#[test_case(
    b"2D6C74304232302D0D739B93E6BE21FEBB557B20",
    "libTorrent (Rakshasa) / rTorrent",
    None
)]
#[test_case(b"-LT0D00-eZ0PwaDDr-~v", "libtorrent (Rasterbar)", None)]
#[test_case(b"-LK0140-ATIV~nbEQAMr", "linkage", None)]
#[test_case(b"2D4C57303030312D31E0B3A0B46F7D4E954F4103", "LimeWire", None)]
#[test_case(b"2D4C50303330322D003833363536393537373030", "Lphant", None)]
#[test_case(b"2D535A323133322D000000000000000000000000", "Shareaza", None)]
#[test_case(b"-ST0117-01234567890!", "SymTorrent", None)]
#[test_case(b"-TR0006-01234567890!", "Transmission", None)]
#[test_case(b"-TR072Z-zihst5yvg22f", "Transmission", None)]
#[test_case(b"-TR0072-8vd6hrmp04an", "Transmission", None)]
#[test_case(b"-TT210w-dq!nWf~Qcext", "TuoTu", None)]
#[test_case(b"2D5554313730422D928446441DB0A094A01C01E5", "\u{00B5}Torrent", None)]
#[test_case(
    b"2D5647323634342D4FD62CDA69E235717E3BB94B",
    "\u{54c7}\u{560E} (Vagaa)",
    None
)]
#[test_case(b"-WY0300-6huHF5Pr7Vde", "FireTorrent", None)]
#[test_case(b"-PC251Q-6huHF5Pr7Vde", "CacheLogic", None)]
#[test_case(b"-KG2450-BDEw8OM14Hk6", "KGet", None)]
//
// "Shadow-style clients"
//
#[test_case(b"A--------YMyoBPXYy2L", "ABC", Some(None))]
#[test_case(
    b"413236392D2D2D2D345077199FAEC4A673BECA01",
    "ABC",
    Some(Some("2.6.9"))
)]
#[test_case(b"A310--001v5Gysr4NxNK", "ABC", Some(Some("3.1.0")))]
#[test_case(b"T03C-----6tYolxhVUFS", "BitTornado", Some(Some("0.3.12")))]
#[test_case(b"T03I--008gY6iB6Aq27C", "BitTornado", Some(Some("0.3.18")))]
#[test_case(b"T0390----5uL5NvjBe2z", "BitTornado", Some(Some("0.3.9")))]
#[test_case(b"R100--003hR6s07XWcov", "Tribler", Some(Some("1.0.0")))]
#[test_case(b"R37---003uApHy851-Pq", "Tribler", Some(Some("3.7")))]
//
// "Simple-style clients"
//
#[test_case(b"417A75726575730000000000000000A076F0AEF7", "Azureus", None)]
#[test_case(b"2D2D2D2D2D417A757265757354694E7A2A6454A7", "Azureus", None)]
#[test_case(b"2D4733416E6F6E796D6F757370E8D9CB30250AD4", "G3 Torrent", None)]
#[test_case(
    b"6172636C696768742E68652EA5860C157A5ADC35",
    "Hurricane Electric",
    None
)]
#[test_case(b"Pando-6B511B691CAC2E", "Pando", None)]
#[test_case(b"2D55543137302D00AF8BC5ACCC4631481EB3EB60", "\u{00B5}Torrent", None)]
//
// "Mainline-style clients"
//
#[test_case(b"M5-0-7--9aa757efd5be", "Mainline", Some(Some("5.0.7")))]
// todo: how do they parse 0000000000000000000000004C53441933104277 ???
#[test_case(b"S3-1-0-0--0123456789", "Amazon AWS S3", Some(Some("1.0.0")))]
//
// "Version substring-style clients"
//
#[test_case(b"4269744C657430319AEA4E02A09E318D70CCF47D", "Bitlet", None)]
#[test_case(b"-BOWP05-EPICNZOGQPHP", "BitsOnWheels", None)]
#[test_case(b"Mbrst1-1-32e3c394b43", "Burst!", None)]
#[test_case(b"OP7685f2c1495b1680bf", "Opera", None)]
#[test_case(b"O100634008270e29150a", "Opera", None)]
#[test_case(b"00455253416E6F6E796D6F757382BE4275024AE3", "Rufus", None)]
#[test_case(b"444E413031303030DD01C9B2DA689E6E02803E91", "BitTorrent DNA", None)]
#[test_case(b"BTM21abcdefghijklmno", "BTuga Revolution", None)]
#[test_case(b"4150302E3730726333302D3E3EB87B31F241DBFE", "AllPeers", None)]
#[test_case(b"45787420EC7CC30033D7801FEEB713FBB0557AC4", "External Webseed", None)]
#[test_case(b"QVOD00541234567890AB", "QVOD", None)]
#[test_case(b"TB100----abcdefghijk", "Top-BT", None)]
//
// BitComet/Lord/Spirit
//
#[test_case(b"6578626300387A4463102D6E9AD6723B339F35A9", "BitComet", None)]
#[test_case(b"6578626300384C4F52443200048ECED57BD71028", "BitLord", None)]
#[test_case(b"4D342D302D322D2D6898D9D0CAF25E4555445030", "BitSpirit?", None)]
#[test_case(b"000242539B7ED3E058A8384AA748485454504254", "BitSpirit", None)]
#[test_case(b"000342530724889644C595308A5FF2CA55445030", "BitSpirit", None)]
//
// "Misc clients"
//
#[test_case(b"TIX0137-i6i6f0i5d5b7", "Tixati", None)]
// folx is actually azureus style
#[test_case(b"2D464C3039C6F22D5F436863327A6D792E283867", "folx", Some(Some("0.x")))]
#[test_case(b"-KT22B1-695754334315", "KTorrent", None)]
#[test_case(b"-KT2140-584815613993", "KTorrent", None)]
#[test_case(
    b"2D554D3135313130C964BE6F15CA71EF02AF2DD7",
    "\u{00B5}Torrent Mac",
    None
)]
#[test_case(b"2D4D47314372302D3234705F6436000055673362", "MediaGet", None)]
#[test_case(b"-#@0000-Em6o1EmvwLtD", "Invalid PeerID", None)]
#[test_case(b"2D4D47323111302D3234705F6436706E55673362", "MediaGet", None)]
#[test_case(b"-AN2171-nr17R1h19O7n", "Ares", None)]
#[test_case(b"2D55543334302D000971FDE48C3688D2023506FC", "\u{00B5}Torrent", None)]
//
// Unknown clients
//
// some of the clients are actually known, truly unknown are tested below
#[test_case(
    b"2D464435315DC72D37426772646B4C3850434239",
    "Free Download Manager",
    Some(Some("5.1"))
)] //
#[test_case(
    b"2D4249313730302D66466D324E356B5848335068",
    "BiglyBT",
    Some(Some("1.7.0.0"))
)]
//
// "WebTorrent"
//
#[test_case(b"-WW0000-Em6o1EmvwLtD", "WebTorrent", Some(Some("0.0")))]
#[test_case(b"-WW0100-Em6o1EmvwLtD", "WebTorrent", Some(Some("1.0")))]
#[test_case(b"-WW1000-Em6o1EmvwLtD", "WebTorrent", Some(Some("10.0")))]
#[test_case(b"-WW0001-Em6o1EmvwLtD", "WebTorrent", Some(Some("0.1")))]
#[test_case(b"-WW0010-Em6o1EmvwLtD", "WebTorrent", Some(Some("0.10")))]
#[test_case(b"-WW0011-Em6o1EmvwLtD", "WebTorrent", Some(Some("0.11")))]
#[test_case(b"-WW1011-Em6o1EmvwLtD", "WebTorrent", Some(Some("10.11")))]
#[test_case(b"-WW1111-Em6o1EmvwLtD", "WebTorrent", Some(Some("11.11")))]
//
// "WebTorrent Desktop"
//
#[test_case(b"-WD0007-Em6o1EmvwLtD", "WebTorrent Desktop", Some(Some("0.7")))]
fn webtorrent_test(peer_id_bytes: &[u8], client_name: &str, test_version: Option<Option<&str>>) {
    let peer_id = PeerId::try_from(raw_or_hex(peer_id_bytes).as_slice()).unwrap();
    let parsed = parse(peer_id).unwrap();
    assert_eq!(parsed.client, client_name);

    if let Some(known_version) = test_version {
        assert_eq!(
            parsed.version.unwrap(),
            known_version.map(ToOwned::to_owned)
        );
    }
}

#[test_case(b"B5546F7272656E742F3330323520202020202020")]
#[test_case(b"0000000000000000317DA32F831FF041A515FE3C")]
#[test_case(b"000000DF05020020100020200008000000004028")]
#[test_case(b"0000000000000000F106CE44F179A2498FAC614F")]
#[test_case(b"E7F163BB0E5FCD35005C09A11BC274C42385A1A0")]
// 2D464435315DC72D37426772646B4C3850434239 is FD
// 2D4249313730302D66466D324E356B5848335068 is BiglyBT
fn webtorrent_unknown_test(peer_id_bytes: &[u8]) {
    let peer_id = PeerId::try_from(raw_or_hex(peer_id_bytes).as_slice()).unwrap();
    let e = parse(peer_id).unwrap_err();
    assert_eq!(e, ClientParsingError::UnknownClient);
}
