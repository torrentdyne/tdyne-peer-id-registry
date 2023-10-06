use crate::errors::VersionParsingError;
use crate::version_utils::base10;
use std::fmt;
use tdyne_peer_id::PeerId;

#[derive(Debug, Copy, Clone)]
pub(crate) struct OneBase10Unknown(pub u8);

impl fmt::Display for OneBase10Unknown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.x", self.0)
    }
}

impl TryFrom<PeerId> for OneBase10Unknown {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, b1, ..] = peer_id.0;

        Ok(OneBase10Unknown(base10(b1)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"-FL51FF-xxxxxxxxxxxx");
        let s = OneBase10Unknown::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "5.x");

        let peer_id = PeerId::from(b"-FL10xx-xxxxxxxxxxxx");
        let s = OneBase10Unknown::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "1.x");

        let peer_id = PeerId::from(b"-MG21xx-xxxxxxxxxxxx");
        let s = OneBase10Unknown::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "2.x");
    }
}
