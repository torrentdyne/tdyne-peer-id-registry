use crate::errors::VersionParsingError;

use crate::client_styles::azureus::version_styles::three_base16::ThreeBase16;
use std::fmt;
use tdyne_peer_id::PeerId;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub(crate) enum Suffix {
    Alpha,
    Beta,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct ThreeBase16AlphaBeta(pub u8, pub u8, pub u8, pub Option<Suffix>);

impl fmt::Display for ThreeBase16AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suffix = match self.3 {
            Some(Suffix::Alpha) => " (Alpha)",
            Some(Suffix::Beta) => " (Beta)",
            None => "",
        };

        write!(f, "{}.{}.{}{}", self.0, self.1, self.2, suffix)
    }
}

impl TryFrom<PeerId> for ThreeBase16AlphaBeta {
    type Error = VersionParsingError;

    fn try_from(peer_id: PeerId) -> Result<Self, Self::Error> {
        let [_dash, _id1, _id2, _b1, _b2, _b3, letter, ..] = peer_id.0;

        let base = ThreeBase16::try_from(peer_id)?;

        let alphabeta = match letter {
            b'A' => Some(Suffix::Alpha),
            b'B' => Some(Suffix::Beta),
            _ => None,
        };

        Ok(ThreeBase16AlphaBeta(base.0, base.1, base.2, alphabeta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
        let peer_id = PeerId::from(b"-XX1AFA-xxxxxxxxxxxx");
        let s = ThreeBase16AlphaBeta::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "1.10.15 (Alpha)");

        let peer_id = PeerId::from(b"-XX1AF0-xxxxxxxxxxxx");
        let s = ThreeBase16AlphaBeta::try_from(peer_id).unwrap().to_string();
        assert_eq!(s, "1.10.15");
    }
}
