use crate::errors::VersionParsingError;
use crate::version_utils::{base10, base62};
use std::fmt;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Suffix {
    Beta,
    Dev,
}

impl fmt::Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Beta => write!(f, " (Beta)"),
            Self::Dev => write!(f, " (Dev)"),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Transmission {
    Pre1(u8),
    Pre3(u8, u8, Option<Suffix>),
    Current(u8, u8, u8, Option<Suffix>),
}

impl fmt::Display for Transmission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pre1(v) => write!(f, "0.{v}"),

            Self::Pre3(v1, v2, maybe_s) => {
                write!(f, "{v1}.{v2}")?;
                if let Some(s) = maybe_s {
                    s.fmt(f)?;
                }
                Ok(())
            }

            Self::Current(v1, v2, v3, maybe_s) => {
                write!(f, "{v1}.{v2}.{v3}")?;
                if let Some(s) = maybe_s {
                    s.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}

impl TryFrom<PeerId> for Transmission {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, b1, b2, b3, b4, ..] = peer_id.0;

        // Transmission parses its own peer ID here:
        // https://github.com/transmission/transmission/blob/ce39b01dd2dfa1352730035c75ef2b6771c8ace2/libtransmission/clients.cc#L418

        if &[b1, b2] == b"00" {
            Ok(Transmission::Pre1(base10(b3)? * 10 + base10(b4)?))
        } else {
            let suffix = match b4 {
                b'Z' => Some(Suffix::Dev),
                b'X' | b'B' => Some(Suffix::Beta),
                b'0' => None,
                other => return Err(VersionParsingError::UnknownTransmissionReleaseType(other)),
            };

            if b1 < b'3' {
                Ok(Transmission::Pre3(
                    base10(b1)?,
                    base10(b2)? * 10 + base10(b3)?,
                    suffix,
                ))
            } else {
                Ok(Transmission::Current(
                    base62(b1)?,
                    base62(b2)?,
                    base62(b3)?,
                    suffix,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(b"-TR0006-xxxxxxxxxxxx", "0.6")]
    #[test_case(b"-TR0072-xxxxxxxxxxxx", "0.72")]
    #[test_case(b"-TR111Z-xxxxxxxxxxxx", "1.11 (Dev)")]
    #[test_case(b"-TR1330-xxxxxxxxxxxx", "1.33")]
    #[test_case(b"-TR133X-xxxxxxxxxxxx", "1.33 (Beta)")]
    #[test_case(b"-TR4040-xxxxxxxxxxxx", "4.0.4")]
    #[test_case(b"-TR404Z-xxxxxxxxxxxx", "4.0.4 (Dev)")]
    #[test_case(b"-TR404B-xxxxxxxxxxxx", "4.0.4 (Beta)")]
    fn test_version(peer_id_bytes: &[u8], version: &str) {
        let peer_id = PeerId::try_from(peer_id_bytes).unwrap();
        let s = Transmission::try_from(peer_id).unwrap().to_string();
        assert_eq!(&s, version);
    }
}
