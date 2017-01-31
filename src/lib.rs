// Copyright 2016  Jonas mg
// See the 'AUTHORS' file at the top-level directory for a full list of authors.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Handles ASCII characters.

pub mod table;
mod errors;

use errors::*;

/// Defines the methods for ASCII operations on characters.
pub trait Check {
    /// `is_letter` checks whether it is an ASCII letter (a-z / A-Z).
    fn is_letter(self) -> bool;
    /// `is_lower` checks whether it is an ASCII lower case letter (a-z).
    fn is_lower(self) -> bool;
    /// `is_upper` checks whether it is an ASCII upper case letter (A-Z).
    fn is_upper(self) -> bool;
    /// `is_digit` checks whether it is an ASCII digit (0-9).
    fn is_digit(self) -> bool;
    /// `is_space` checks whether it is an ASCII space character
    /// (Space, Horizontal Tab, Line Feed, Vertical Tab, Form Feed, Carriage Return).
    fn is_space(self) -> bool;

    /// `is_control` checks whether it is an ASCII control character.
    /// The control characters are unprintable control codes and are used
    /// to control peripherals such as printers.
    fn is_control(self) -> bool;

    /// `is_printable` checks whether it is an ASCII printable character.
    /// The printable characters are common for all the different variations
    /// of the ASCII table; represent letters, digits, punctuation marks,
    /// and a few miscellaneous symbols.
    fn is_printable(self) -> bool;

    /// `is_us_ascii` checks whether it is an US-ASCII character.
    fn is_us_ascii(self) -> bool;

    /// `is_extended` checks whether it is an extended ASCII character.
    fn is_extended(self) -> bool;
}

impl Check for u8 {
    fn is_letter(self) -> bool {
        match self {
            b'a'...b'z' | b'A'...b'Z' => true,
            _ => false,
        }
    }

    fn is_lower(self) -> bool {
        match self {
            b'a'...b'z' => true,
            _ => false,
        }
    }

    fn is_upper(self) -> bool {
        match self {
            b'A'...b'Z' => true,
            _ => false,
        }
    }

    fn is_digit(self) -> bool {
        match self {
            b'0'...b'9' => true,
            _ => false,
        }
    }

    fn is_space(self) -> bool {
        match self {
            table::SPACE | 0x09...0x0D => true,
            _ => false,
        }
    }

    fn is_control(self) -> bool {
        match self {
            0x00...0x1F | 0x7F => true,
            _ => false,
        }
    }

    fn is_printable(self) -> bool {
        match self {
            0x20...0x7E => true,
            _ => false,
        }
    }

    fn is_us_ascii(self) -> bool {
        match self {
            0x00...0x7F => true,
            _ => false,
        }
    }

    fn is_extended(self) -> bool {
        match self {
            0x80...0xFF => true,
            _ => false,
        }
    }
}

impl Check for char {
    fn is_letter(self) -> bool {
        match self {
            'a'...'z' | 'A'...'Z' => true,
            _ => false,
        }
    }

    fn is_lower(self) -> bool {
        match self {
            'a'...'z' => true,
            _ => false,
        }
    }

    fn is_upper(self) -> bool {
        match self {
            'A'...'Z' => true,
            _ => false,
        }
    }

    fn is_digit(self) -> bool {
        match self {
            '0'...'9' => true,
            _ => false,
        }
    }

    fn is_space(self) -> bool {
        match self {
            table::SPACE_AS_CHAR |
            '\u{9}'...'\u{D}' => true,
            _ => false,
        }
    }

    fn is_control(self) -> bool {
        match self as u8 {
            0x00...0x1F | 0x7F => true,
            _ => false,
        }
    }

    fn is_printable(self) -> bool {
        match self as u8 {
            0x20...0x7E => true,
            _ => false,
        }
    }

    fn is_us_ascii(self) -> bool {
        match self as u8 {
            0x00...0x7F => true,
            _ => false,
        }
    }

    fn is_extended(self) -> bool {
        match self as u8 {
            0x80...0xFF => true,
            _ => false,
        }
    }
}

/// Reports an error wheter the string has a non-ASCII character or any ASCII
/// control character.
pub fn check_ascii_printable(name: &str) -> Result<(), AsciiError> {
    let mut i: usize = 0;

    for byte in name.bytes() {
        match byte {
            0x20...0x7E => (),
            0x00...0x1F | 0x7F => return Err(AsciiError::ControlChar(i + 1)),
            _ => {
                match name[i..].chars().next() {
                    Some(v) => return Err(AsciiError::NonAscii(v)),
                    None => unreachable!(),
                }
            }
        }
        i = i + 1;
    }

    Ok(())
}

// == Tests
//

#[cfg(test)]
mod tests {
    use errors::*;
    use super::*;

    #[test]
    fn test_letter() {
        assert!(Check::is_letter(b'a'));
        assert!(Check::is_letter('j'));
        assert!(Check::is_letter('Z'));

        assert_eq!('0'.is_letter(), false);
    }

    #[test]
    fn test_lower() {
        assert!(Check::is_lower(b'a'));
        assert!(Check::is_lower('z'));

        assert_eq!(Check::is_lower('J'), false);
    }

    #[test]
    fn test_upper() {
        assert!(Check::is_upper(b'A'));
        assert!(Check::is_upper('Z'));

        assert_eq!(Check::is_upper('j'), false);
    }

    #[test]
    fn test_digit() {
        assert!(Check::is_digit(b'0'));
        assert!(Check::is_digit('5'));
        assert!(Check::is_digit('9'));

        assert_eq!(Check::is_digit('a'), false)
    }

    #[test]
    fn test_space() {
        assert!(Check::is_space(table::HT));
        assert!(Check::is_space(table::SPACE));

        assert_eq!(Check::is_space(b'a'), false);
        assert_eq!(Check::is_space('-'), false);
    }

    #[test]
    fn test_control() {
        assert_eq!(Check::is_control('a'), false);
    }

    #[test]
    fn test_printable() {
        assert!(Check::is_printable(b'J'));
        assert!(Check::is_printable('0'));
        assert!(Check::is_printable('-'));

        assert_eq!(Check::is_printable(table::DELETE), false);
    }

    #[test]
    fn test_us_ascii() {
        assert!(Check::is_us_ascii('a'));

        assert_eq!(Check::is_us_ascii('€'), false);
    }

    #[test]
    fn test_extended() {
        assert!(Check::is_extended('€'));

        assert_eq!(Check::is_extended('a'), false);
    }

    #[test]
    fn test_check_ascii_printable() {
        check_ascii_printable("aeiou").unwrap();

        assert_eq!(check_ascii_printable("äeiou"),
                   Err(AsciiError::NonAscii('ä')));
        assert_eq!(check_ascii_printable("aeïou"),
                   Err(AsciiError::NonAscii('ï')));
        assert_eq!(check_ascii_printable("aeioü"),
                   Err(AsciiError::NonAscii('ü')));
        assert_eq!(check_ascii_printable("foo€bar"),
                   Err(AsciiError::NonAscii('€')));
        assert_eq!(check_ascii_printable("foo♦bar"),
                   Err(AsciiError::NonAscii('♦')));

        assert_eq!(check_ascii_printable("foo\tbar"),
                   Err(AsciiError::ControlChar(4)));
    }
}
