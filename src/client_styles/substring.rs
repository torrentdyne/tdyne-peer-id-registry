// those are clients that are matched with substrings

use crate::client_styles::types::ClientStyle;
use crate::errors::{ClientParsingError, VersionParsingError};
use crate::known_clients::KnownClient;
use crate::version::Version;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Substring {
    // apparently this version misformed its peer id
    // see https://web.archive.org/web/20130603184652/http://forum.utorrent.com/viewtopic.php?pid=260927
    UTorrent170RC,
    Azureus1,
    Azureus2032,
    Aria,
    BitTorrentPlusII,
    BitTorrentPlus,
    BitTyrantAzureusMod,
    BlizzardDownloader,
    BTugaXp,
    BtGetit,
    DeadmanWalking,
    Deadman,
    ExternalWebseed,
    G3Torrent,
    GreedBt271,
    HurricaneElectric,
    HttpSeed,
    JvTorrent,
    Limewire,
    MartiniMan,
    Pando,
    PeerApp,
    SimpleBt,
    Swarmy,
    Teeweety,
    TorrentTopia,
    XanTorrent,
    BitTorrentDna,
    Opera,
    Burst,
    TurboBt,
    BtProtocolDaemon,
    Plus,
    Xbt,
    EXeem,
    BitsOnWheels,
    MlDonkey,
    Bitlet,
    AllPeers,
    BTugaRevolution,
    Rufus,
    BitMagnet,
    Qvod,
    TopBt,
    Tixati,
    UTorrentMac,
    UTorrent,
}

pub(crate) struct Variant(Substring, &'static [u8], usize);

// todo: can actually be a static trie (e.g. generate a bunch of nested matches)
const VARIANTS: [Variant; 52] = [
    Variant(Substring::UTorrent170RC, b"-UT170-", 0),
    Variant(Substring::Azureus1, b"Azureus", 0),
    Variant(Substring::Azureus2032, b"Azureus", 5),
    Variant(Substring::Aria, b"-aria2-", 0),
    Variant(Substring::BitTorrentPlusII, b"PRC.P---", 0),
    Variant(Substring::BitTorrentPlus, b"P87.P---", 0),
    Variant(Substring::BitTorrentPlus, b"S587Plus", 0),
    Variant(Substring::BitTyrantAzureusMod, b"AZ2500BT", 0),
    Variant(Substring::BlizzardDownloader, b"BLZ", 0),
    Variant(Substring::BtGetit, b"BG", 10),
    Variant(Substring::BTugaXp, b"btuga", 0),
    Variant(Substring::BTugaXp, b"BTuga", 5),
    Variant(Substring::BTugaXp, b"oernu", 0),
    Variant(Substring::DeadmanWalking, b"BTDWV-", 0),
    Variant(Substring::Deadman, b"Deadman Walking-", 0),
    Variant(Substring::ExternalWebseed, b"Ext", 0),
    Variant(Substring::G3Torrent, b"-G3", 0),
    Variant(Substring::GreedBt271, b"271-", 0),
    Variant(Substring::HurricaneElectric, b"arclight", 0),
    Variant(Substring::HttpSeed, b"-WS", 0),
    Variant(Substring::JvTorrent, b"10-------", 0),
    Variant(Substring::Limewire, b"LIME", 0),
    Variant(Substring::MartiniMan, b"martini", 0),
    Variant(Substring::Pando, b"Pando", 0),
    Variant(Substring::PeerApp, b"PEERAPP", 0),
    Variant(Substring::SimpleBt, b"btfans", 4),
    Variant(Substring::Swarmy, b"a00---0", 0),
    Variant(Substring::Swarmy, b"a02---0", 0),
    Variant(Substring::Teeweety, b"T00---0", 0),
    Variant(Substring::TorrentTopia, b"346-", 0),
    Variant(Substring::XanTorrent, b"DansClient", 0),
    Variant(Substring::BitTorrentDna, b"DNA", 0),
    Variant(Substring::Opera, b"OP", 0),
    Variant(Substring::Opera, b"O", 0),
    Variant(Substring::Burst, b"Mbrst", 0),
    Variant(Substring::TurboBt, b"turbobt", 0),
    Variant(Substring::BtProtocolDaemon, b"btpd", 0),
    Variant(Substring::Plus, b"Plus", 0),
    Variant(Substring::Xbt, b"XBT", 0),
    Variant(Substring::BitsOnWheels, b"-BOW", 0),
    Variant(Substring::EXeem, b"eX", 0),
    Variant(Substring::MlDonkey, b"-ML", 0),
    Variant(Substring::Bitlet, b"BitLet", 0),
    Variant(Substring::AllPeers, b"AP", 0),
    Variant(Substring::BTugaRevolution, b"BTM", 0),
    Variant(Substring::Rufus, b"RS", 2),
    Variant(Substring::BitMagnet, b"BM", 2),
    Variant(Substring::Qvod, b"QVOD", 0),
    Variant(Substring::TopBt, b"TB", 0),
    Variant(Substring::Tixati, b"TIX", 0),
    Variant(Substring::UTorrentMac, b"-UM", 0),
    Variant(Substring::UTorrent, b"-UT", 0),
];

impl ClientStyle for Substring {
    fn does_match(_peer_id: PeerId) -> bool {
        // this is a dummy method, there is no difference between checking if a client
        // belongs to the substring-matching category and finding a match
        true
    }

    fn parse(peer_id: PeerId) -> Result<Self, ClientParsingError> {
        for Variant(client, substr, offset) in VARIANTS {
            if substr == &peer_id.0[offset..(offset + substr.len())] {
                return Ok(client);
            }
        }
        Err(ClientParsingError::UnknownClient)
    }

    fn parse_version(self, _peer_id: PeerId) -> Result<Option<Version>, VersionParsingError> {
        match self {
            Self::UTorrent170RC => Ok(Some(Version::Fixed("1.7.0 RC"))),
            Self::Azureus1 => Ok(Some(Version::Fixed("1"))),
            Self::Azureus2032 => Ok(Some(Version::Fixed("2.0.3.2"))),
            Self::Aria => Ok(Some(Version::Fixed("2"))),
            Self::BitTorrentPlusII => Ok(Some(Version::Fixed("II"))),
            Self::GreedBt271 => Ok(Some(Version::Fixed("2.7.1"))),
            // those don't have version according to webtorrent
            Self::BitTorrentPlus
            | Self::BitTyrantAzureusMod
            | Self::BlizzardDownloader
            | Self::BTugaXp
            | Self::BtGetit
            | Self::DeadmanWalking
            | Self::Deadman
            | Self::ExternalWebseed
            | Self::HurricaneElectric
            | Self::HttpSeed
            | Self::JvTorrent
            | Self::Limewire
            | Self::MartiniMan
            | Self::Pando
            | Self::PeerApp
            | Self::SimpleBt
            | Self::Swarmy
            | Self::Teeweety
            | Self::TorrentTopia
            | Self::XanTorrent
            | Self::G3Torrent => Ok(None),
            // these do, but their versioning schemes aren't implemented (yet?)
            Self::BitTorrentDna
            | Self::Opera
            | Self::Burst
            | Self::TurboBt
            | Self::BtProtocolDaemon
            | Self::Plus
            | Self::Xbt
            | Self::EXeem
            | Self::BitsOnWheels
            | Self::MlDonkey
            | Self::Bitlet
            | Self::AllPeers
            | Self::BTugaRevolution
            | Self::Rufus
            | Self::BitMagnet
            | Self::Qvod
            // apparently almost shadow
            // https://github.com/webtorrent/bittorrent-peerid/blob/f8457f24ef95b3e5eaa134bf0b5e264580c0eb09/index.js#L422C6-L422C63
            | Self::TopBt
            | Self::Tixati
            // not sure why webtorrent included those here
            // todo: double check why
            | Self::UTorrentMac
            | Self::UTorrent => Ok(Some(Version::Unknown)),
        }
    }

    fn to_canonical(self) -> KnownClient {
        use KnownClient as KC;
        match self {
            Self::UTorrent170RC => KC::UTorrent,
            Self::Aria => KC::Aria,
            Self::BitTorrentPlus => KC::BitTorrentPlus,
            Self::BitTyrantAzureusMod => KC::BitTyrantAzureusMod,
            Self::BlizzardDownloader => KC::BlizzardDownloader,
            Self::BTugaXp => KC::BTugaXp,
            Self::BtGetit => KC::BtGetit,
            Self::DeadmanWalking => KC::DeadmanWalking,
            Self::Deadman => KC::Deadman,
            Self::ExternalWebseed => KC::ExternalWebseed,
            Self::G3Torrent => KC::G3Torrent,
            Self::GreedBt271 => KC::GreedBt,
            Self::HurricaneElectric => KC::HurricaneElectric,
            Self::HttpSeed => KC::HttpSeed,
            Self::JvTorrent => KC::JvTorrent,
            Self::Limewire => KC::Limewire,
            Self::MartiniMan => KC::MartiniMan,
            Self::PeerApp => KC::PeerApp,
            Self::SimpleBt => KC::SimpleBt,
            Self::Teeweety => KC::Teeweety,
            Self::TorrentTopia => KC::TorrentTopia,
            Self::BitTorrentDna => KC::BitTorrentDna,
            Self::Opera => KC::Opera,
            Self::Burst => KC::Burst,
            Self::TurboBt => KC::TurboBt,
            Self::BtProtocolDaemon => KC::BtProtocolDaemon,
            Self::Plus => KC::Plus,
            Self::Xbt => KC::Xbt,
            Self::EXeem => KC::EXeem,
            Self::BitsOnWheels => KC::BitsOnWheels,
            Self::MlDonkey => KC::MlDonkey,
            Self::AllPeers => KC::AllPeers,
            Self::BTugaRevolution => KC::BTugaRevolution,
            Self::Rufus => KC::Rufus,
            Self::BitMagnet => KC::BitMagnet,
            Self::Qvod => KC::Qvod,
            Self::TopBt => KC::TopBt,
            Self::Tixati => KC::Tixati,
            Self::Azureus1 => KC::Azureus,
            Self::Azureus2032 => KC::Azureus,
            Self::BitTorrentPlusII => KnownClient::BitTorrentPlus,
            Self::Pando => KnownClient::Pando,
            Self::Swarmy => KnownClient::Swarmy,
            Self::XanTorrent => KnownClient::XanTorrent,
            Self::Bitlet => KnownClient::Bitlet,
            Self::UTorrentMac => KnownClient::UTorrentMac,
            Self::UTorrent => KnownClient::UTorrent,
        }
    }
}
