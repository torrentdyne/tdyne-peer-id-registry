use crate::client_styles::types::ClientStyle;
use crate::errors::{ClientParsingError, VersionParsingError};
use crate::known_clients::KnownClient;
use crate::version::Version;
use crate::version_utils::base10;
use std::fmt;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Mainline {
    #[allow(clippy::enum_variant_names)]
    Mainline,
    QueenBee,
    AmazonAWSS3,
}

impl ClientStyle for Mainline {
    fn does_match(peer_id: PeerId) -> bool {
        // it's a relatively rare style and is not very well defined, so I just try to match
        // known clients directly
        match peer_id.0 {
            [b'M' | b'Q', b1, ..] if b1.is_ascii_digit() => true,
            [b'S', b'3', b'-', b1, ..] if b1.is_ascii_digit() => true,
            _ => false,
        }
    }

    fn parse(peer_id: PeerId) -> Result<Self, ClientParsingError> {
        match peer_id.0 {
            [b'M', b1, ..] if b1.is_ascii_digit() => Ok(Self::Mainline),
            [b'Q', b1, ..] if b1.is_ascii_digit() => Ok(Self::QueenBee),
            [b'S', b'3', b'-', b1, ..] if b1.is_ascii_digit() => Ok(Self::AmazonAWSS3),
            _ => {
                let mainline_prefix = &peer_id.0[0..4];
                Err(ClientParsingError::UnknownMainlineClientPrefix(
                    mainline_prefix.try_into().unwrap(),
                ))
            }
        }
    }

    fn parse_version(self, peer_id: PeerId) -> Result<Option<Version>, VersionParsingError> {
        // this should be guaranteed in the parse above
        assert!(peer_id.0[1].is_ascii_digit());

        let start = if peer_id.0[0] == b'S' { 3 } else { 1 };

        let mut v1 = 0u8;
        let mut v2 = 0u8;
        let mut v3 = 0u8;

        let vs = [&mut v1, &mut v2, &mut v3];
        let mut current_v_idx = 0;

        for i in start..peer_id.0.len() {
            match peer_id.0[i] {
                b'-' => {
                    current_v_idx += 1;
                    if current_v_idx == vs.len() {
                        break;
                    }
                }
                b => {
                    let v = base10(b)?;
                    *vs[current_v_idx] = vs[current_v_idx]
                        .checked_mul(10)
                        .and_then(|x| x.checked_add(v))
                        .ok_or(VersionParsingError::VersionOverflow)?;
                }
            }
        }

        Ok(Some(Version::Mainline(MainlineVersion(v1, v2, v3))))
    }

    fn to_canonical(self) -> KnownClient {
        match self {
            Self::Mainline => KnownClient::Mainline,
            Self::QueenBee => KnownClient::QueenBee,
            Self::AmazonAWSS3 => KnownClient::AmazonAWSS3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct MainlineVersion(pub u8, pub u8, pub u8);

impl fmt::Display for MainlineVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(v1, v2, v3) = self;
        write!(f, "{v1}.{v2}.{v3}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(b"M4-3-6--xxxxxxxxxxxx", "4.3.6")]
    #[test_case(b"M4-20-8-xxxxxxxxxxxx", "4.20.8")]
    #[test_case(b"M4-20-42-xxxxxxxxxxx", "4.20.42")]
    #[test_case(b"M4-20-120-xxxxxxxxxx", "4.20.120")]
    #[test_case(b"S3-1-0-0--0123456789", "1.0.0")]
    fn test_mainline_version(peer_id_bytes: &[u8; 20], version: &str) {
        let peer_id = PeerId::from(peer_id_bytes);
        assert_eq!(
            Mainline::Mainline
                .parse_version(peer_id)
                .unwrap()
                .unwrap()
                .to_string(),
            version.to_string()
        );
    }
}
