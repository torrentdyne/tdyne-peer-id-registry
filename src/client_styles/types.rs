use crate::errors::{ClientParsingError, VersionParsingError};
use crate::known_clients::KnownClient;
use crate::version::Version;
use std::fmt;
use tdyne_peer_id::PeerId;

pub(crate) trait ClientStyle: fmt::Debug + Copy {
    fn does_match(peer_id: PeerId) -> bool;
    fn parse(peer_id: PeerId) -> Result<Self, ClientParsingError>;
    fn parse_version(self, peer_id: PeerId) -> Result<Option<Version>, VersionParsingError>;
    fn to_canonical(self) -> KnownClient;
}
