use crate::client_styles::azureus::Azureus;
use crate::client_styles::mainline::Mainline;
use crate::client_styles::shadow::Shadow;
use crate::client_styles::substring::Substring;
use crate::client_styles::types::ClientStyle;
use crate::errors::{ClientParsingError, VersionParsingError};
use crate::known_clients::KnownClient;
use crate::version::{BitCometMod, Version};
use crate::version_utils::base10;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Client {
    AzureusStyle(Azureus),
    ShadowStyle(Shadow),
    MainlineStyle(Mainline),
    Substring(Substring),
    PossibleBitSpirit,
    BitSpirit,
    BitComet,
    BitLord,
}

impl Client {
    pub fn parse_version(self, peer_id: PeerId) -> Result<Option<Version>, VersionParsingError> {
        match self {
            Self::AzureusStyle(x) => x.parse_version(peer_id),
            Self::ShadowStyle(x) => x.parse_version(peer_id),
            Self::MainlineStyle(x) => x.parse_version(peer_id),
            Self::Substring(x) => x.parse_version(peer_id),

            Self::PossibleBitSpirit => Ok(None),
            Self::BitSpirit => parse_bitspirit_version(peer_id),
            c @ (Self::BitComet | Self::BitLord) => parse_bitcomet_bitlord_version(c, peer_id),
        }
    }

    pub fn to_canonical(self) -> KnownClient {
        match self {
            Self::AzureusStyle(x) => x.to_canonical(),
            Self::ShadowStyle(x) => x.to_canonical(),
            Self::MainlineStyle(x) => x.to_canonical(),
            Self::Substring(x) => x.to_canonical(),
            Self::PossibleBitSpirit => KnownClient::PossibleBitSpirit,
            Self::BitSpirit => KnownClient::BitSpirit,
            Self::BitComet => KnownClient::BitComet,
            Self::BitLord => KnownClient::BitLord,
        }
    }
}

fn check_bitspirit(peer_id: PeerId) -> Option<Client> {
    if &peer_id.0[2..4] == b"BS" {
        Some(Client::BitSpirit)
    } else {
        None
    }
}

fn parse_bitspirit_version(peer_id: PeerId) -> Result<Option<Version>, VersionParsingError> {
    let v1 = base10(peer_id.0[1])?.max(1);
    Ok(Some(Version::BitSpirit(v1)))
}

fn parse_bitcomet_bitlord_version(
    client: Client,
    peer_id: PeerId,
) -> Result<Option<Version>, VersionParsingError> {
    let m = match peer_id.0 {
        [b'e', b'x', b'b', b'c', ..] => None,
        [b'F', b'U', b'T', b'B', ..] => Some(BitCometMod::Solidox),
        [b'x', b'U', b'T', b'B', ..] => Some(BitCometMod::Mod2),
        [b1, b2, b3, b4, ..] => {
            return Err(VersionParsingError::UnexpectedBitCometBitLordHeader(
                b1, b2, b3, b4,
            ))
        }
    };
    let v1 = base10(peer_id.0[4])?;
    let v2 = if matches!(client, Client::BitLord) && v1 > 0 {
        base10(peer_id.0[5])?
    } else {
        base10(peer_id.0[5])? * 10 + base10(peer_id.0[6])?
    };
    Ok(Some(Version::BitCometBitLord(v1, v2, m)))
}

fn check_bitcomet(peer_id: PeerId) -> Option<Client> {
    if peer_id.0.starts_with(b"exbc")
        || peer_id.0.starts_with(b"FUTB")
        || peer_id.0.starts_with(b"xUTB")
    {
        return if &peer_id.0[6..10] == b"LORD" {
            Some(Client::BitLord)
        } else {
            Some(Client::BitComet)
        };
    }
    None
}

impl TryFrom<PeerId> for Client {
    type Error = ClientParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        if peer_id.0.ends_with(b"UDP0") || peer_id.0.ends_with(b"HTTPBT") {
            // apparently BitSpirit and BitComet have a special spoof mode, so I need
            // to check for their special encodings, lest I misidentify them further down
            if let Some(client) = check_bitspirit(peer_id) {
                return Ok(client);
            }
            if let Some(client) = check_bitcomet(peer_id) {
                return Ok(client);
            }

            return Ok(Self::PossibleBitSpirit);
        }

        if Azureus::does_match(peer_id) {
            let client = Azureus::parse(peer_id)?;

            // webtorrent (and apparently A LOT of code going decade+ back, see
            // https://github.com/search?q=%2F%28%3F-i%29%22bLAde%22%2F&type=code&p=1 )
            // applies some heuristics here to weed out "fake ZipTorrent clients" or
            // "misidentified BitTorrent 6.0":
            // https://github.com/webtorrent/bittorrent-peerid/blob/f8457f24ef95b3e5eaa134bf0b5e264580c0eb09/index.js#L46
            // Chances are those clients are long dead, so I just ignore those hacks here

            return Ok(Self::AzureusStyle(client));
        }

        if Shadow::does_match(peer_id) {
            let client = Shadow::parse(peer_id)?;
            return Ok(Self::ShadowStyle(client));
        }

        if Mainline::does_match(peer_id) {
            let client = Mainline::parse(peer_id)?;
            return Ok(Self::MainlineStyle(client));
        }

        // bitspirit/bitcomet check outside of their spoof mode
        if let Some(client) = check_bitspirit(peer_id) {
            return Ok(client);
        }
        if let Some(client) = check_bitcomet(peer_id) {
            return Ok(client);
        }

        Substring::parse(peer_id).map(Self::Substring)
    }
}
