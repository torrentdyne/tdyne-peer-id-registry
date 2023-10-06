use crate::client_styles::types::ClientStyle;
use crate::errors::{ClientParsingError, VersionParsingError};
use crate::known_clients::KnownClient;
use crate::version::Version;
use crate::version_utils::base62;
use std::fmt;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Shadow {
    Abc,
    OspreyPermaseed,
    BTQueue,
    Tribler,
    Shad0w,
    BitTornado,
    UPnPNat,
}

// you can find the list of tags in build.rs
include!(concat!(env!("OUT_DIR"), "/codegen_tags_shadow.rs"));

fn is_base62(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z')
}

impl ClientStyle for Shadow {
    fn does_match(peer_id: PeerId) -> bool {
        // Here I deviate form webtorrent. I check that the first letter is a known tag,
        // that the first six symbols ("header" in the description) are a (possibly empty)
        // sequence of base64 symbols ending in dashes. Unlike mainline encoding, Shadow
        // shouldn't have symbols after dashes.
        //
        // I tried to filter the tag by just an uppercase ASCII, but it then captures stuff
        // like `Pando-`, which does fit the pattern.
        if !TAGS.contains_key(&peer_id.0[0]) {
            return false;
        }

        // TopBT does match the criteria, but it's not really a shadow-style client
        if &peer_id.0[0..=5] == b"TB100-" {
            return false;
        }

        let mut padding_started = false;

        if peer_id.0[5] != b'-' {
            return false;
        }

        // I use base62 instead of the original base64 because - has two meanings, and both
        // . and - should be fairly rare in version numbers (how many projects reach v63?)
        for b in &peer_id.0[1..5] {
            match b {
                b'-' if padding_started => (),
                b'-' if !padding_started => padding_started = true,
                x if is_base62(*x) && !padding_started => (),
                x if is_base62(*x) && padding_started => return false,
                _ => return false,
            }
        }

        true
    }

    fn parse(peer_id: PeerId) -> Result<Self, ClientParsingError> {
        let prefix = peer_id.0[0];
        TAGS.get(&prefix)
            .copied()
            .ok_or(ClientParsingError::UnknownShadowStylePrefix(prefix))
    }

    fn parse_version(self, peer_id: PeerId) -> Result<Option<Version>, VersionParsingError> {
        let [_tag, b1, b2, b3, b4, ..] = peer_id.0;

        if b1 == b'-' {
            return Ok(None);
        }

        let v1 = base62(b1)?;

        let raw_bs = [b2, b3, b4];

        let mut rest_vs: [Option<u8>; 3] = [None; 3];

        for i in 0..rest_vs.len() {
            rest_vs[i] = match raw_bs[i] {
                b'-' => None,
                other => Some(base62(other)?),
            }
        }

        // strip last zero in case it's just a release type designator
        if let Some(x @ Some(0)) = rest_vs.last_mut() {
            *x = None;
        }

        Ok(Some(Version::Shadow(ShadowVersion(v1, rest_vs))))
    }

    fn to_canonical(self) -> KnownClient {
        use KnownClient as KC;

        match self {
            Self::Abc => KC::Abc,
            Self::OspreyPermaseed => KC::OspreyPermaseed,
            Self::BTQueue => KC::BTQueue,
            Self::Tribler => KC::Tribler,
            Self::Shad0w => KC::Shad0w,
            Self::BitTornado => KC::BitTornado,
            Self::UPnPNat => KC::UPnPNAT,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
// split into a value and array to guarantee that the version is not empty (the entire thing
// should be None if it is)
pub struct ShadowVersion(pub u8, pub [Option<u8>; 3]);

impl fmt::Display for ShadowVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.1 {
            [None, ..] => write!(f, "{}", self.0),
            [Some(v1), None, ..] => write!(f, "{}.{}", self.0, v1),
            [Some(v1), Some(v2), None] => write!(f, "{}.{}.{}", self.0, v1, v2),
            [Some(v1), Some(v2), Some(v3)] => write!(f, "{}.{}.{}.{}", self.0, v1, v2, v3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(b"T03A0-xxxxxxxxxxxxxx", true)]
    #[test_case(b"T03A--xxxxxxxxxxxxxx", true)]
    #[test_case(b"T03---xxxxxxxxxxxxxx", true)]
    #[test_case(b"T0----xxxxxxxxxxxxxx", true)]
    #[test_case(b"T-----xxxxxxxxxxxxxx", true)]
    #[test_case(b"X0----xxxxxxxxxxxxxx", false)] // unknown tags don't match
    #[test_case(b"T0-2--xxxxxxxxxxxxxx", false)]
    #[test_case(b"T0!---xxxxxxxxxxxxxx", false)]
    fn test_match(peer_id_bytes: &[u8; 20], should_match: bool) {
        let peer_id = PeerId::from(peer_id_bytes);
        assert_eq!(Shadow::does_match(peer_id), should_match);
    }

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"T03A0-xxxxxxxxxxxxxx");
        let s = Shadow::Tribler
            .parse_version(peer_id)
            .unwrap()
            .unwrap()
            .to_string();
        assert_eq!(s, "0.3.10");
    }

    #[test]
    fn test_four() {
        let peer_id = PeerId::from(b"T03A5-xxxxxxxxxxxxxx");
        let s = Shadow::Tribler
            .parse_version(peer_id)
            .unwrap()
            .unwrap()
            .to_string();
        assert_eq!(s, "0.3.10.5");
    }
}
