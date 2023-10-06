use std::fmt;

///
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VersionParsingError {
    /// Returned when the library expects the parsed client's version format to
    /// include a byte corresponding to an ASCII base 10 number, but the actual bytes
    /// is outside of `b'0'..=b'9'` range. Includes the offending byte.
    DigitNotBase10(u8),
    /// Same as DigitNotBase10, but for base 16.      
    DigitNotBase16(u8),
    /// Same as DigitNotBase10, but for base 62 (`0-9A-Za-z`).
    DigitNotBase62(u8),
    /// `tdyne_peer_id_registry` expects the versions to fit into `u8`. In some encodings
    /// it's possible to encode a number larger than `[u8::MAX]`; if this happens, the library
    /// returns this error.
    VersionOverflow,
    /// Transmission version encoding includes one byte for a release type (release/beta/dev).
    /// If the actual peer ID has a byte that is not in the Transmission spec, the library
    /// returns this error. Includes the offending byte.
    UnknownTransmissionReleaseType(u8),
    /// BitComet and BitLord have a prefix that is normally checked during client detection.
    /// If for some reason the header is not found during version parsing, this error is returned.
    /// Should never arise if the API is not misused.
    UnexpectedBitCometBitLordHeader(u8, u8, u8, u8),
}

impl fmt::Display for VersionParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DigitNotBase16(x) => {
                write!(f, "Can't decode byte {x} as a base 16 digit",)
            }
            Self::DigitNotBase10(x) => {
                write!(f, "Can't decode byte {x} as a base 10 digit",)
            }
            Self::DigitNotBase62(x) => {
                write!(f, "Can't decode byte {x} as a base 62 digit",)
            }
            Self::VersionOverflow => {
                write!(f, "Version number is too large")
            }
            Self::UnknownTransmissionReleaseType(x) => {
                write!(
                    f,
                    "Can't decode Transmission release type from the byte {x}"
                )
            }
            Self::UnexpectedBitCometBitLordHeader(b1, b2, b3, b4) => {
                write!(
                    f,
                    "\"{}{}{}{}\" doesn't match a known BitLord or BitComet prefix",
                    ascii_or_byte(*b1),
                    ascii_or_byte(*b2),
                    ascii_or_byte(*b3),
                    ascii_or_byte(*b4)
                )
            }
        }
    }
}

impl std::error::Error for VersionParsingError {}

///
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClientParsingError {
    /// The library detects the encoding style of a given peer ID first and then matches
    /// the encoded client identificator against the registry of all known identifiers.
    /// If the peer ID matches Azureus style, but the actual two-byte client identifier
    /// is unknown, the library returns this error.
    UnknownAzureusStylePrefix([u8; 2]),
    /// Same but for Shad0w-style clients.
    UnknownShadowStylePrefix(u8),
    /// Same but for Mainline style clients.
    UnknownMainlineClientPrefix([u8; 4]),
    /// Returned when the library fails to recognise the client.
    UnknownClient,
}

fn ascii_or_byte(b: u8) -> String {
    if b.is_ascii_alphanumeric() {
        char::from(b).to_string()
    } else {
        format!("\\x{:02}", b)
    }
}

impl fmt::Display for ClientParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownAzureusStylePrefix([b1, b2]) => {
                write!(
                    f,
                    "\"{}{}\" doesn't match a known Azureus-style prefix",
                    ascii_or_byte(*b1),
                    ascii_or_byte(*b2)
                )
            }
            Self::UnknownShadowStylePrefix(b) => {
                write!(
                    f,
                    "\"{}\" doesn't match a known Shadow-style prefix",
                    ascii_or_byte(*b)
                )
            }
            Self::UnknownMainlineClientPrefix([b1, b2, b3, b4]) => {
                write!(
                    f,
                    "\"{}{}{}{}\" doesn't match a known Mainline-style prefix",
                    ascii_or_byte(*b1),
                    ascii_or_byte(*b2),
                    ascii_or_byte(*b3),
                    ascii_or_byte(*b4)
                )
            }
            Self::UnknownClient => {
                write!(f, "Unknown client")
            }
        }
    }
}

impl std::error::Error for ClientParsingError {}
