use crate::errors::VersionParsingError;

pub(crate) fn base16(b: u8) -> Result<u8, VersionParsingError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(10 + (b - b'A')),
        // note: the danger with a..f is that there is also base62 encoding
        //       that uses a..f for the higher range. Presumably an error trying to parse
        //       anything outside A..F|a..f range would make it clear that a client actually
        //       uses base64 and not base16
        _ => Err(VersionParsingError::DigitNotBase16(b)),
    }
}

pub(crate) fn base10(b: u8) -> Result<u8, VersionParsingError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        _ => Err(VersionParsingError::DigitNotBase10(b)),
    }
}

// todo: document
// the original "spec" here http://forums.degreez.net/viewtopic.php?t=7070
// uses "-" both as padding and as the last symbol in the alphabet, which is pretty problematic.
// I assume that versions don't reach "-" and exclude it from the alphabet.
pub(crate) fn base62(b: u8) -> Result<u8, VersionParsingError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'Z' => Ok((b'9' - b'0' + 1) + (b - b'A')),
        b'a'..=b'z' => Ok((b'9' - b'0' + 1) + (b'Z' - b'A' + 1) + (b - b'a')),
        _ => Err(VersionParsingError::DigitNotBase62(b)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base16() {
        for test_byte in u8::MIN..=u8::MAX {
            let test_char = char::from(test_byte);
            let char_parsed = test_char.to_digit(16).map(|x| u8::try_from(x).unwrap());
            match char_parsed {
                Some(valid_b16) if test_char.is_uppercase() || test_char.is_ascii_digit() => {
                    assert_eq!(base16(test_byte).unwrap(), valid_b16);
                }
                _ => {
                    assert_eq!(
                        base16(test_byte).unwrap_err(),
                        VersionParsingError::DigitNotBase16(test_byte)
                    )
                }
            }
        }
    }

    #[test]
    fn test_base10() {
        for b in u8::MIN..=u8::MAX {
            match char::from(b).to_digit(10) {
                Some(d) => {
                    assert_eq!(base10(b).unwrap(), u8::try_from(d).unwrap());
                }
                None => {
                    assert_eq!(
                        base10(b).unwrap_err(),
                        VersionParsingError::DigitNotBase10(b)
                    )
                }
            }
        }
    }

    #[test]
    fn test_base62() {
        let alphabet = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        for i in 0u8..62u8 {
            assert_eq!(base62(alphabet[i as usize]).unwrap(), i);
        }
    }
}
