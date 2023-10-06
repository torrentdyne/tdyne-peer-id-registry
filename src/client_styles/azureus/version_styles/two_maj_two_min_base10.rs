use crate::errors::VersionParsingError;
use crate::version_utils::base10;
use std::fmt;
use tdyne_peer_id::PeerId;

#[derive(Debug, Copy, Clone)]
pub(crate) struct TwoMajTwoMinBase10(pub u8, pub u8);

impl TryFrom<PeerId> for TwoMajTwoMinBase10 {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, b11, b12, b21, b22, ..] = peer_id.0;
        Ok(TwoMajTwoMinBase10(
            base10(b11)? * 10 + base10(b12)?,
            base10(b21)? * 10 + base10(b22)?,
        ))
    }
}

impl fmt::Display for TwoMajTwoMinBase10 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"-XX0125-xxxxxxxxxxxx");
        let s = TwoMajTwoMinBase10::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "1.25");
    }
}
