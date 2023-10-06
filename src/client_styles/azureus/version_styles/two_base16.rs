use crate::errors::VersionParsingError;

use crate::version_utils::base16;
use std::fmt;
use tdyne_peer_id::PeerId;

#[derive(Debug, Copy, Clone)]
pub(crate) struct TwoBase16(pub u8, pub u8);

impl fmt::Display for TwoBase16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl TryFrom<PeerId> for TwoBase16 {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, b1, b2, ..] = peer_id.0;
        Ok(TwoBase16(base16(b1)?, base16(b2)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"-XX5111-xxxxxxxxxxxx");
        let s = TwoBase16::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "5.1");
    }
}
