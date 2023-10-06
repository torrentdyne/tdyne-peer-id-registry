use crate::errors::VersionParsingError;

use crate::version_utils::base16;
use std::fmt;
use tdyne_peer_id::PeerId;

#[derive(Debug, Copy, Clone)]
pub(crate) struct ThreeBase16(pub u8, pub u8, pub u8);

impl fmt::Display for ThreeBase16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl TryFrom<PeerId> for ThreeBase16 {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, b1, b2, b3, ..] = peer_id.0;
        Ok(ThreeBase16(base16(b1)?, base16(b2)?, base16(b3)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"-XX1AF0-xxxxxxxxxxxx");
        let s = ThreeBase16::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "1.10.15");
    }
}
