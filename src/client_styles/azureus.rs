pub mod version_styles;

use crate::client_styles::azureus::version_styles::four_base16::FourBase16;
use crate::client_styles::azureus::version_styles::one_base10_unknown::OneBase10Unknown;
use crate::client_styles::azureus::version_styles::three_base16::ThreeBase16;
use crate::client_styles::azureus::version_styles::three_base16_alpha_beta::ThreeBase16AlphaBeta;
use crate::client_styles::azureus::version_styles::transmission::Transmission;
use crate::client_styles::azureus::version_styles::two_base16::TwoBase16;
use crate::client_styles::azureus::version_styles::two_maj_two_min_base10::TwoMajTwoMinBase10;
use crate::client_styles::azureus::version_styles::Styles as VersionStyles;
use crate::client_styles::types::ClientStyle;
use crate::errors::{ClientParsingError, VersionParsingError};
use crate::known_clients::KnownClient;
use crate::version::Version;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Azureus {
    AresThreeDigits,
    AresFourDigits,
    Avicora,
    BitPump,
    Artemis,
    Vuze,
    BitBuddy,
    BitComet,
    BitTorrentSDK,
    BitFlu,
    Btg,
    BitKitten,
    BitRocket,
    BTSlave,
    BitTorrent,
    BitWombat,
    BittorrentX,
    ShareazaPlus,
    EnhancedCTorrent,
    CTorrent,
    PropogateDataClient,
    Deluge,
    EBit,
    ElectricSheep,
    FileCroc,
    FlashGet,
    FreeboxBitTorrent,
    FreeDownloadManager,
    FoxTorrentRedSwoosh,
    GetRight,
    GSTorrent,
    Halite,
    Hydranode,
    KGet,
    KTorrent,
    LeechCraft,
    LhAbc,
    Linkage,
    Lphant,
    LibtorrentRasterbar,
    LibTorrentRakshasa,
    LimeWire,
    MonoTorrent,
    MooPolice,
    Miro,
    MoonlightTorrent,
    BTNextEvolution,
    NetTransport,
    OneSwarm,
    OmegaTorrent,
    CacheLogic,
    PopcornTime,
    Pando,
    PeerProject,
    PHoeniX,
    QBittorrent,
    QqDownload,
    RumTorrent,
    Retriever,
    RezTorrent,
    ShareazaAlphaBeta,
    SwiftBit,
    Xunlei,
    ShareNET,
    BitSpirit,
    SwarmScope,
    SymTorrent,
    SharkTorrent,
    Shareaza,
    TorrentGO,
    TorrentDotNET,
    Transmission,
    TorrentStorm,
    TuoTu,
    ULeecher,
    UTorrentEmbedded,
    UTorrent,
    UTorrentMac,
    UTorrentWeb,
    WebTorrentDesktop,
    Bitlet,
    WebTorrent,
    FireTorrent,
    Vagaa,
    XanTorrent,
    Xfplay,
    XTorrent,
    ZipTorrent,
    ATorrent,
    Zona,
    InvalidPeerId,
    Folx,
    MediaGet,
    BiglyBt,
    BiglyBtAndroid,
}

// you can find the list of tags in build.rs
include!(concat!(env!("OUT_DIR"), "/codegen_tags_azureus.rs"));

impl ClientStyle for Azureus {
    fn does_match(peer_id: PeerId) -> bool {
        if peer_id.0[0] != b'-' {
            return false;
        }

        if peer_id.0[7] == b'-' {
            // Bits on Wheels uses the pattern -BOWxxx-yyyyyyyyyyyy, which looks like Azureus
            // but isn't
            if &peer_id.0[1..=3] == b"BOW" {
                return false;
            }

            return true;
        }

        // webtorrent hacks around those particular clients' quirks, see
        // https://github.com/webtorrent/bittorrent-peerid/blob/f8457f24ef95b3e5eaa134bf0b5e264580c0eb09/lib/utils.js#L5
        let tag: &[u8; 2] = peer_id.0[1..3].try_into().unwrap();
        matches!(tag, b"FG" | b"LH" | b"NE" | b"KT" | b"SP")
    }

    fn parse(peer_id: PeerId) -> Result<Self, ClientParsingError> {
        let az_prefix = &peer_id.0[1..3];
        let candidate =
            TAGS.get(az_prefix)
                .copied()
                .ok_or(ClientParsingError::UnknownAzureusStylePrefix(
                    az_prefix.try_into().unwrap(),
                ))?;

        match candidate {
            // see https://github.com/BiglySoftware/BiglyBT/commit/9bc529b86f95003ab2dd664066dc490fc20cd2ea
            // BiglyBT uses first byte after the last header dash to identify Android
            Self::BiglyBt => {
                if peer_id.0[8] == b'A' {
                    Ok(Self::BiglyBtAndroid)
                } else {
                    Ok(Self::BiglyBt)
                }
            }
            other => Ok(other),
        }
    }

    fn parse_version(self, peer_id: PeerId) -> Result<Option<Version>, VersionParsingError> {
        use Version as V;
        use VersionStyles as VS;

        let v = match self {
            Self::AresThreeDigits
            | Self::Deluge
            | Self::ElectricSheep
            | Self::Halite
            | Self::KTorrent
            | Self::Linkage
            | Self::LibtorrentRasterbar
            | Self::LibTorrentRakshasa
            | Self::MooPolice
            | Self::BTNextEvolution
            | Self::QBittorrent
            | Self::BitSpirit
            | Self::TuoTu => Some(V::Azureus(VS::ThreeBase16(ThreeBase16::try_from(peer_id)?))),
            Self::BitPump
            | Self::BitComet
            | Self::EnhancedCTorrent
            | Self::FlashGet
            | Self::Lphant
            | Self::WebTorrentDesktop
            | Self::WebTorrent => Some(V::Azureus(VS::TwoMajTwoMinBase10(
                TwoMajTwoMinBase10::try_from(peer_id)?,
            ))),
            Self::BitBuddy
            | Self::BitRocket
            | Self::CTorrent
            | Self::GetRight
            | Self::GSTorrent
            | Self::CacheLogic
            | Self::SymTorrent
            | Self::XTorrent => Some(V::Unknown),
            Self::BitFlu | Self::LimeWire => None,
            Self::BitTorrent
            | Self::UTorrentEmbedded
            | Self::UTorrent
            | Self::UTorrentMac
            | Self::UTorrentWeb => Some(V::Azureus(VS::ThreeBase16AlphaBeta(
                ThreeBase16AlphaBeta::try_from(peer_id)?,
            ))),
            // todo: xfplay identification is wrong, see
            //       https://github.com/transmission/transmission/pull/256
            Self::Transmission | Self::Xfplay => Some(V::Azureus(VS::Transmission(
                Transmission::try_from(peer_id)?,
            ))),
            Self::Folx | Self::MediaGet => Some(V::Azureus(VS::OneBase10Unknown(
                OneBase10Unknown::try_from(peer_id)?,
            ))),
            Self::AresFourDigits
            | Self::Avicora
            | Self::Artemis
            | Self::Vuze
            | Self::BitTorrentSDK
            | Self::Btg
            | Self::BitKitten
            | Self::BTSlave
            | Self::BitWombat
            | Self::BittorrentX
            | Self::ShareazaPlus
            | Self::PropogateDataClient
            | Self::EBit
            | Self::FileCroc
            | Self::FreeboxBitTorrent
            | Self::FoxTorrentRedSwoosh
            | Self::Hydranode
            | Self::KGet
            | Self::LeechCraft
            | Self::LhAbc
            | Self::MonoTorrent
            | Self::Miro
            | Self::MoonlightTorrent
            | Self::NetTransport
            | Self::OneSwarm
            | Self::OmegaTorrent
            | Self::PopcornTime
            | Self::Pando
            | Self::PeerProject
            | Self::PHoeniX
            | Self::QqDownload
            | Self::RumTorrent
            | Self::Retriever
            | Self::RezTorrent
            | Self::ShareazaAlphaBeta
            | Self::SwiftBit
            | Self::Xunlei
            | Self::ShareNET
            | Self::SwarmScope
            | Self::SharkTorrent
            | Self::Shareaza
            | Self::TorrentGO
            | Self::TorrentDotNET
            | Self::TorrentStorm
            | Self::ULeecher
            | Self::Bitlet
            | Self::FireTorrent
            | Self::Vagaa
            | Self::XanTorrent
            | Self::ZipTorrent
            | Self::ATorrent
            | Self::Zona
            | Self::InvalidPeerId
            | Self::BiglyBt
            | Self::BiglyBtAndroid => {
                Some(V::Azureus(VS::FourBase16(FourBase16::try_from(peer_id)?)))
            }
            Self::FreeDownloadManager => {
                Some(V::Azureus(VS::TwoBase16(TwoBase16::try_from(peer_id)?)))
            }
        };

        Ok(v)
    }

    fn to_canonical(self) -> KnownClient {
        use KnownClient as KC;

        match self {
            Self::AresThreeDigits | Self::AresFourDigits => KC::Ares,
            Self::Avicora => KC::Avicora,
            Self::BitPump => KC::BitPump,
            Self::Artemis => KC::Artemis,
            Self::Vuze => KC::Vuze,
            Self::BitBuddy => KC::BitBuddy,
            Self::BitComet => KC::BitComet,
            Self::BitTorrentSDK => KC::BitTorrentSDK,
            Self::BitFlu => KC::BitFlu,
            Self::Btg => KC::Btg,
            Self::BitKitten => KC::BitKitten,
            Self::BitRocket => KC::BitRocket,
            Self::BTSlave => KC::BTSlave,
            Self::BitTorrent => KC::BitTorrent,
            Self::BitWombat => KC::BitWombat,
            Self::BittorrentX => KC::BittorrentX,
            Self::ShareazaPlus => KC::ShareazaPlus,
            Self::EnhancedCTorrent => KC::EnhancedCTorrent,
            Self::CTorrent => KC::CTorrent,
            Self::PropogateDataClient => KC::PropogateDataClient,
            Self::Deluge => KC::Deluge,
            Self::EBit => KC::EBit,
            Self::ElectricSheep => KC::ElectricSheep,
            Self::FileCroc => KC::FileCroc,
            Self::FlashGet => KC::FlashGet,
            Self::FreeboxBitTorrent => KC::FreeboxBitTorrent,
            Self::FreeDownloadManager => KC::FreeDownloadManager,
            Self::FoxTorrentRedSwoosh => KC::FoxTorrentRedSwoosh,
            Self::GetRight => KC::GetRight,
            Self::GSTorrent => KC::GSTorrent,
            Self::Halite => KC::Halite,
            Self::Hydranode => KC::Hydranode,
            Self::KGet => KC::KGet,
            Self::KTorrent => KC::KTorrent,
            Self::LeechCraft => KC::LeechCraft,
            Self::LhAbc => KC::LhAbc,
            Self::Linkage => KC::Linkage,
            Self::Lphant => KC::Lphant,
            Self::LibtorrentRasterbar => KC::LibtorrentRasterbar,
            Self::LibTorrentRakshasa => KC::LibTorrentRakshasa,
            Self::LimeWire => KC::LimeWire,
            Self::MonoTorrent => KC::MonoTorrent,
            Self::MooPolice => KC::MooPolice,
            Self::Miro => KC::Miro,
            Self::MoonlightTorrent => KC::MoonlightTorrent,
            Self::BTNextEvolution => KC::BTNextEvolution,
            Self::NetTransport => KC::NetTransport,
            Self::OneSwarm => KC::OneSwarm,
            Self::OmegaTorrent => KC::OmegaTorrent,
            Self::CacheLogic => KC::CacheLogic,
            Self::PopcornTime => KC::PopcornTime,
            Self::Pando => KC::Pando,
            Self::PeerProject => KC::PeerProject,
            Self::PHoeniX => KC::PHoeniX,
            Self::QBittorrent => KC::QBittorrent,
            Self::QqDownload => KC::QqDownload,
            Self::RumTorrent => KC::RumTorrent,
            Self::Retriever => KC::Retriever,
            Self::RezTorrent => KC::RezTorrent,
            Self::ShareazaAlphaBeta => KC::ShareazaAlphaBeta,
            Self::SwiftBit => KC::SwiftBit,
            Self::Xunlei => KC::Xunlei,
            Self::ShareNET => KC::ShareNET,
            Self::BitSpirit => KC::BitSpirit,
            Self::SwarmScope => KC::SwarmScope,
            Self::SymTorrent => KC::SymTorrent,
            Self::SharkTorrent => KC::SharkTorrent,
            Self::Shareaza => KC::Shareaza,
            Self::TorrentGO => KC::TorrentGO,
            Self::TorrentDotNET => KC::TorrentDotNET,
            Self::Transmission => KC::Transmission,
            Self::TorrentStorm => KC::TorrentStorm,
            Self::TuoTu => KC::TuoTu,
            Self::ULeecher => KC::ULeecher,
            Self::UTorrentEmbedded => KC::UTorrentEmbedded,
            Self::UTorrent => KC::UTorrent,
            Self::UTorrentMac => KC::UTorrentMac,
            Self::UTorrentWeb => KC::UTorrentWeb,
            Self::WebTorrentDesktop => KC::WebTorrentDesktop,
            Self::Bitlet => KC::Bitlet,
            Self::WebTorrent => KC::WebTorrent,
            Self::FireTorrent => KC::FireTorrent,
            Self::Vagaa => KC::Vagaa,
            Self::XanTorrent => KC::XanTorrent,
            Self::Xfplay => KC::Xfplay,
            Self::XTorrent => KC::XTorrent,
            Self::ZipTorrent => KC::ZipTorrent,
            Self::ATorrent => KC::ATorrent,
            Self::Zona => KC::Zona,
            Self::InvalidPeerId => KC::InvalidPeerId,
            Self::Folx => KC::Folx,
            Self::MediaGet => KC::MediaGet,
            Self::BiglyBt => KC::BiglyBt,
            Self::BiglyBtAndroid => KC::BiglyBtAndroid,
        }
    }
}
