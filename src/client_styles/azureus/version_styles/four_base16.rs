use crate::errors::VersionParsingError;
use crate::version_utils::base16;
use std::fmt;
use tdyne_peer_id::PeerId;

#[derive(Debug, Copy, Clone)]
pub(crate) struct FourBase16(pub u8, pub u8, pub u8, pub u8);

impl fmt::Display for FourBase16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl TryFrom<PeerId> for FourBase16 {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, b1, b2, b3, b4, ..] = peer_id.0;
        Ok(FourBase16(
            base16(b1)?,
            base16(b2)?,
            base16(b3)?,
            base16(b4)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"-XX10AF-xxxxxxxxxxxx");
        let s = FourBase16::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "1.0.10.15");
    }
}
