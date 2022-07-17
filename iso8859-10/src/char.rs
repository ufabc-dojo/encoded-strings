//! This module implements the types to represent a character of ISO8859-10 characters.

use core::{fmt, mem};

use super::map;

/// A single ISO8859-10 character.
///
/// # Validity
/// A `IsoLatin6Char` is valid if it is a valid well defined ISO8859-10 character or ASCII control
/// codes.
///
/// ## Why ASCII control codes are valid?
/// Although ISO8859-10 does not define ASCII control codes values (`0x00` to `0x1F`), we consider
/// them valid for convenience.
///
/// Since these code values are considered undefined by the standard, the decision on what to do
/// with them is implementation defined. Its commom to implement this standard considering those
/// code values like we do.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IsoLatin6Char(u8);

// Public API
impl IsoLatin6Char {
    /// Returns `true` if this character has the `Alphabetic` property.
    ///
    /// `Alphabetic` is described in Chapter 4 (Character Properties) of the [Unicode Standard] and
    /// specified in the [Unicode Character Database][ucd] [`DerivedCoreProperties.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`DerivedCoreProperties.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from(b'a')?.is_alphabetic());
    ///
    /// assert!(!IsoLatin6Char::try_from(b'!')?.is_alphabetic());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_alphabetic(&self) -> bool {
        matches!(self.0, 0x41..=0x5A
            | 0x61..=0x7A
            | 0xA1..=0xA6
            | 0xA8..=0xAC
            | 0xAE..=0xAF
            | 0xB1..=0xB6
            | 0xB8..=0xBC
            | 0xBE..=0xFF)
    }

    /// Returns `true` if this character satisfies either [`is_alphabetic`] or [`is_numeric`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from('A')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('þ')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('Æ')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('1')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('9')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('0')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('Ą')?.is_alphanumeric());
    /// assert!(IsoLatin6Char::try_from('ð')?.is_alphanumeric());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_numeric()
    }

    /// Returns `true` if this character has the general category for control codes.
    ///
    /// Control codes (code points with the general category of `Cc`) are described in Chapter 4
    /// (Character Properties) of the [Unicode Standard] and specified in the [Unicode Character
    /// Database][ucd] [`UnicodeData.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`UnicodeData.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from('\0')?.is_control());
    /// assert!(!IsoLatin6Char::try_from('q').is_control());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_control(&self) -> bool {
        matches!(self.0, 0x00..=0x1F | 0x7F)
    }

    /// Checks if a `char` is a digit in the given radix.
    ///
    /// A 'radix' here is sometimes also called a 'base'. A radix of two
    /// indicates a binary number, a radix of ten, decimal, and a radix of
    /// sixteen, hexadecimal, to give some common values. Arbitrary
    /// radices are supported.
    ///
    /// Compared to [`is_numeric()`], this function only recognizes the characters
    /// `0-9`, `a-z` and `A-Z`.
    ///
    /// 'Digit' is defined to be only the following characters:
    ///
    /// * `0-9`
    /// * `a-z`
    /// * `A-Z`
    ///
    /// For a more comprehensive understanding of 'digit', see [`is_numeric()`].
    ///
    /// [`is_numeric()`]: #method.is_numeric
    ///
    /// # Panics
    ///
    /// Panics if given a radix larger than 36.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from('1')?.is_digit(10));
    /// assert!(IsoLatin6Char::try_from('f')?.is_digit(16));
    /// assert!(!IsoLatin6Char::try_from('f')?.is_digit(10));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Passing a large radix, causing a panic:
    ///
    /// ```should_panic
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// // this panics
    /// IsoLatin6Char::try_from('1')?.is_digit(37);
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_digit(&self, radix: u8) -> bool {
        // If not a digit, a number greater than radix will be created.
        let mut digit = (self.0).wrapping_sub(b'0');
        if radix > 10 {
            assert!(radix <= 36, "is_digit: radix is too high (maximum 36)");
            if digit < 10 {
                return true;
            }

            // Force the 6th bit to be set to ensure ascii is lower case.
            digit = (self.0 | 0b10_0000).wrapping_sub(b'a').saturating_add(10);
        }

        digit < radix
    }

    /// Returns `true` if this character has one of the general categories for numbers.
    ///
    /// The general categories for numbers (`Nd` for decimal digits, `Nl` for letter-like numeric
    /// characters, and `No` for other numeric characters) are specified in the [Unicode Character
    /// Database][ucd] [`UnicodeData.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`UnicodeData.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from('1')?.is_numeric());
    /// assert!(IsoLatin6Char::try_from('7')?.is_numeric());
    /// assert!(IsoLatin6Char::try_from('0')?.is_numeric());
    /// assert!(!IsoLatin6Char::try_from('K')?.is_numeric());
    /// assert!(!IsoLatin6Char::try_from('ø')?.is_numeric());
    /// assert!(!IsoLatin6Char::try_from('ð')?.is_numeric());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_numeric(&self) -> bool {
        matches!(self.0, 0x30..=0x39)
    }

    /// Returns `true` if this character has the `White_Space` property.
    ///
    /// `White_Space` is specified in the [Unicode Character Database][ucd] [`PropList.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`PropList.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from(' ')?.is_whitespace());
    ///
    /// // line break
    /// assert!(IsoLatin6Char::try_from('\n')?.is_whitespace());
    ///
    /// // a non-breaking space
    /// assert!(IsoLatin6Char::try_from('\u{A0}')?.is_whitespace());
    ///
    /// assert!(!IsoLatin6Char::try_from('Æ')?.is_whitespace());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_whitespace(&self) -> bool {
        matches!(self.0, 0x09 | 0x0A | 0x0C | 0x0D | 0x20 | 0xA0)
    }

    /// Returns `true` if this character has the `Lowercase` property.
    ///
    /// `Lowercase` is described in Chapter 4 (Character Properties) of the [Unicode Standard] and
    /// specified in the [Unicode Character Database][ucd] [`DerivedCoreProperties.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`DerivedCoreProperties.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(IsoLatin6Char::try_from('a')?.is_lowercase());
    /// assert!(IsoLatin6Char::try_from('þ')?.is_lowercase());
    /// assert!(!IsoLatin6Char::try_from('A')?.is_lowercase());
    /// assert!(!IsoLatin6Char::try_from('Þ')?.is_lowercase());
    ///
    /// // The various characters and punctuation do not have case, and so:
    /// assert!(!IsoLatin6Char::try_from('·')?.is_lowercase());
    /// assert!(!IsoLatin6Char::try_from(' ')?.is_lowercase());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_lowercase(&self) -> bool {
        matches!(self.0, 0x61..=0x7A | 0xB1..=0xB6 | 0xB8..=0xBC | 0xBE..=0xBF | 0xE0..=0xFF)
    }

    /// Returns `true` if this character has the `Uppercase` property.
    ///
    /// `Uppercase` is described in Chapter 4 (Character Properties) of the [Unicode Standard] and
    /// specified in the [Unicode Character Database][ucd] [`DerivedCoreProperties.txt`].
    ///
    /// Althought this type is not an Unicode, we use the same database to get the property for the
    /// character symbols.
    ///
    /// [Unicode Standard]: https://www.unicode.org/versions/latest/
    /// [ucd]: https://www.unicode.org/reports/tr44/
    /// [`DerivedCoreProperties.txt`]: https://www.unicode.org/Public/UCD/latest/ucd/DerivedCoreProperties.txt
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// assert!(!IsoLatin6Char::try_from('a')?.is_uppercase());
    /// assert!(!IsoLatin6Char::try_from('þ')?.is_uppercase());
    /// assert!(IsoLatin6Char::try_from('A')?.is_uppercase());
    /// assert!(IsoLatin6Char::try_from('Þ')?.is_uppercase());
    ///
    /// // The various Chinese scripts and punctuation do not have case, and so:
    /// assert!(!IsoLatin6Char::try_from('·')?.is_uppercase());
    /// assert!(!IsoLatin6Char::try_from(' ')?.is_uppercase());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_uppercase(&self) -> bool {
        matches!(self.0, 0x41..=0x5A | 0xA1..=0xA6 | 0xA8..=0xAC | 0xAE..=0xAF | 0xC0..=0xDF)
    }
}

// Public API related to ASCII
impl IsoLatin6Char {
    /// Checks if the value is within the ASCII range.
    ///
    /// # Examples
    ///
    /// ```
    /// use iso8859_10::IsoLatin6Char;
    ///
    /// # fn main() -> Result<(), iso8859_10::char::IsoLatin6CharError> {
    /// let ascii = IsoLatin6Char::try_from('a')?;
    /// let non_ascii = IsoLatin6Char::try_from('æ')?;
    ///
    /// assert!(ascii.is_ascii());
    /// assert!(!non_ascii.is_ascii());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_ascii(&self) -> bool {
        self.0 <= 0x7F
    }
}

impl fmt::Debug for IsoLatin6Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <char as fmt::Debug>::fmt(&self.0.into(), f)
    }
}

impl fmt::Display for IsoLatin6Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <char as fmt::Display>::fmt(&self.0.into(), f)
    }
}

impl fmt::LowerHex for IsoLatin6Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <u8 as fmt::LowerHex>::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for IsoLatin6Char {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <u8 as fmt::UpperHex>::fmt(&self.0, f)
    }
}

impl TryFrom<u8> for IsoLatin6Char {
    type Error = IsoLatin6CharError;

    #[inline]
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use IsoLatin6CharError::*;

        match byte {
            // SAFETY: The representation of `IsoLatin6Char` is a single byte.
            0x00..=0x7F => Ok(unsafe { mem::transmute(byte) }),
            0x80..=0x9F => Err(Undefined),
            0xA0..=0xFF => Ok(unsafe { mem::transmute(byte) }),
        }
    }
}

impl From<IsoLatin6Char> for u8 {
    #[inline]
    fn from(char: IsoLatin6Char) -> u8 {
        char.0
    }
}

impl TryFrom<char> for IsoLatin6Char {
    type Error = IsoLatin6CharError;

    #[inline]
    fn try_from(char: char) -> Result<Self, Self::Error> {
        map_char_to_byte(char).map(Self)
    }
}

impl From<IsoLatin6Char> for char {
    #[inline]
    fn from(char: IsoLatin6Char) -> Self {
        // SAFETY: `IsoLatin6Char` is always valid ISO8859-10 character and all valid values of this
        // type has a valid `char` value
        unsafe { map_byte_to_char_unchecked(char.0) }
    }
}

/// Error type to represent possible reasons for a byte not being a valid [`IsoLatin6Char`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IsoLatin6CharError {
    /// The byte is not defined as a specific character in ISO8859-10 and it's not ASCII control
    /// codes.
    Undefined,
    /// The byte contains a invalid value.
    Invalid,
}

#[allow(dead_code)]
fn map_byte_to_char(byte: u8) -> Result<char, IsoLatin6CharError> {
    use IsoLatin6CharError::*;

    match byte {
        0x00..=0x7F => Ok(byte as char),
        0x80..=0x9F => Err(Undefined),
        c => match map::DECODE_MAP.get(c as usize - 0xA0) {
            Some(0) => Err(Invalid),
            Some(&c) => char::from_u32(c as u32).ok_or(Invalid),
            None => Err(Invalid),
        },
    }
}

/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
/// - The value of the `byte` must not be between `0x7F` and `0x9F`
unsafe fn map_byte_to_char_unchecked(byte: u8) -> char {
    match byte {
        0x00..=0x7F => byte as char,
        c => char::from_u32_unchecked(*map::DECODE_MAP.get_unchecked(c as usize - 0xA0) as u32),
    }
}

fn map_char_to_byte(ch: char) -> Result<u8, IsoLatin6CharError> {
    use IsoLatin6CharError::*;
    let c = ch as u32 as u16;
    match c {
        0x00..=0x7F => Ok(c as u8),
        0x80..=0x9F => Err(Invalid),
        c => {
            let hi = (c >> 8) as usize;
            let lo = (c & 0xFF) as usize;

            let pos = map::HI_MAP
                .get(hi)
                .map(|pos| pos * 0x100 + lo)
                .ok_or(Invalid)?;
            let &code = map::ENCODE_MAP.get(pos).ok_or(Invalid)?;

            if code != 0x0000 {
                Ok(code)
            } else {
                Err(Invalid)
            }
        },
    }
}


#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn is_alphabetic() {
        assert!(IsoLatin6Char(0x41).is_alphabetic());
        assert!(IsoLatin6Char(0x5A).is_alphabetic());
        assert!(IsoLatin6Char(0x61).is_alphabetic());
        assert!(IsoLatin6Char(0x7A).is_alphabetic());
        assert!(IsoLatin6Char(0xA1).is_alphabetic());
        assert!(IsoLatin6Char(0xA6).is_alphabetic());
        assert!(IsoLatin6Char(0xA8).is_alphabetic());
        assert!(IsoLatin6Char(0xAC).is_alphabetic());
        assert!(IsoLatin6Char(0xAE).is_alphabetic());
        assert!(IsoLatin6Char(0xAF).is_alphabetic());
        assert!(IsoLatin6Char(0xB1).is_alphabetic());
        assert!(IsoLatin6Char(0xB6).is_alphabetic());
        assert!(IsoLatin6Char(0xB8).is_alphabetic());
        assert!(IsoLatin6Char(0xBC).is_alphabetic());
        assert!(IsoLatin6Char(0xBE).is_alphabetic());
        assert!(IsoLatin6Char(0xFF).is_alphabetic());

        for byte in 0x80..=0xA0 {
            assert!(!IsoLatin6Char(byte).is_alphabetic());
        }
    }

    #[test]
    fn is_control() {
        for byte in 0x00..=0x1F {
            assert!(IsoLatin6Char(byte).is_control());
        }
        for byte in 0x20..=0xFF {
            assert!(!IsoLatin6Char(byte).is_control());
        }
    }

    #[test]
    fn is_digit() {
        assert!(IsoLatin6Char(b'0').is_digit(10));
        assert!(IsoLatin6Char(b'1').is_digit(2));
        assert!(IsoLatin6Char(b'2').is_digit(3));
        assert!(IsoLatin6Char(b'9').is_digit(10));
        assert!(IsoLatin6Char(b'a').is_digit(16),);
        assert!(IsoLatin6Char(b'A').is_digit(16),);
        assert!(IsoLatin6Char(b'b').is_digit(16),);
        assert!(IsoLatin6Char(b'B').is_digit(16),);
        assert!(IsoLatin6Char(b'A').is_digit(36),);
        assert!(IsoLatin6Char(b'z').is_digit(36),);
        assert!(IsoLatin6Char(b'Z').is_digit(36),);
        assert!(!IsoLatin6Char(b'[').is_digit(36));
        assert!(!IsoLatin6Char(b'`').is_digit(36));
        assert!(!IsoLatin6Char(b'{').is_digit(36));
        assert!(!IsoLatin6Char(b'$').is_digit(36));
        assert!(!IsoLatin6Char(b'@').is_digit(16));
        assert!(!IsoLatin6Char(b'G').is_digit(16));
        assert!(!IsoLatin6Char(b'g').is_digit(16));
        assert!(!IsoLatin6Char(b' ').is_digit(10));
        assert!(!IsoLatin6Char(b'/').is_digit(10));
        assert!(!IsoLatin6Char(b':').is_digit(10));
        assert!(!IsoLatin6Char(b':').is_digit(11));
    }

    #[test]
    fn is_numeric() {
        for byte in b'0'..=b'9' {
            assert!(IsoLatin6Char(byte).is_numeric());
        }
        for byte in 0x00..=0x2F {
            assert!(!IsoLatin6Char(byte).is_numeric());
        }
        for byte in 0x3A..=0xFF {
            assert!(!IsoLatin6Char(byte).is_numeric());
        }
    }

    #[test]
    fn is_whitespace() {
        assert!(IsoLatin6Char(b' ').is_whitespace());
        assert!(IsoLatin6Char(b'\t').is_whitespace());
        assert!(IsoLatin6Char(b'\n').is_whitespace());
        assert!(!IsoLatin6Char(b'a').is_whitespace());
        assert!(!IsoLatin6Char(b'_').is_whitespace());
        assert!(!IsoLatin6Char(b'\0').is_whitespace());
    }

    #[test]
    fn is_uppercase() {
        assert!(IsoLatin6Char(b'A').is_uppercase());
        assert!(IsoLatin6Char(b'Z').is_uppercase());
        assert!(!IsoLatin6Char(b'a').is_uppercase());
        assert!(!IsoLatin6Char(b'z').is_uppercase());
        assert!(!IsoLatin6Char(b'0').is_uppercase());
        assert!(!IsoLatin6Char(b'9').is_uppercase());
        assert!(!IsoLatin6Char(b'_').is_uppercase());
        assert!(!IsoLatin6Char(b'\0').is_uppercase());
    }

    #[test]
    fn is_lowercase() {
        assert!(IsoLatin6Char(b'a').is_lowercase());
        assert!(IsoLatin6Char(b'z').is_lowercase());
        assert!(!IsoLatin6Char(b'A').is_lowercase());
        assert!(!IsoLatin6Char(b'Z').is_lowercase());
        assert!(!IsoLatin6Char(b'0').is_lowercase());
        assert!(!IsoLatin6Char(b'9').is_lowercase());
        assert!(!IsoLatin6Char(b'_').is_lowercase());
        assert!(!IsoLatin6Char(b'\0').is_lowercase());
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;

    static LAST_PART_OF_ISO8859: [char; 96] = [
        '\u{A0}', 'Ą', 'Ē', 'Ģ', 'Ī', 'Ĩ', 'Ķ', '§', 'Ļ', 'Đ', 'Š', 'Ŧ', 'Ž', '\u{AD}', 'Ū', 'Ŋ',
        '°', 'ą', 'ē', 'ģ', 'ī', 'ĩ', 'ķ', '·', 'ļ', 'đ', 'š', 'ŧ', 'ž', '―', 'ū', 'ŋ', 'Ā', 'Á',
        'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Į', 'Č', 'É', 'Ę', 'Ë', 'Ė', 'Í', 'Î', 'Ï', 'Ð', 'Ņ', 'Ō', 'Ó',
        'Ô', 'Õ', 'Ö', 'Ũ', 'Ø', 'Ų', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', 'ā', 'á', 'â', 'ã', 'ä', 'å',
        'æ', 'į', 'č', 'é', 'ę', 'ë', 'ė', 'í', 'î', 'ï', 'ð', 'ņ', 'ō', 'ó', 'ô', 'õ', 'ö', 'ũ',
        'ø', 'ų', 'ú', 'û', 'ü', 'ý', 'þ', 'ĸ',
    ];

    #[test]
    fn debug() {
        let upcase_a = IsoLatin6Char(0x41);
        assert_eq!(format!("{:?}", upcase_a), "'A'");

        let upcase_ash = IsoLatin6Char(0xC6);
        assert_eq!(format!("{:?}", upcase_ash), "'Æ'");

        let upcase_acaudata = IsoLatin6Char(0xA1);
        assert_eq!(format!("{:?}", upcase_acaudata), "'Ą'");
    }

    #[test]
    fn display() {
        let upcase_a = IsoLatin6Char(0x41);
        assert_eq!(format!("{:?}", upcase_a), "A");

        let upcase_ash = IsoLatin6Char(0xC6);
        assert_eq!(format!("{:?}", upcase_ash), "Æ");

        let upcase_acaudata = IsoLatin6Char(0xA1);
        assert_eq!(format!("{:?}", upcase_acaudata), "Ą");
    }

    #[test]
    fn lowerhex() {
        for byte in 0x00..=0xFF {
            let char = IsoLatin6Char(byte);
            assert_eq!(format!("{:x}", char), format!("{:x}", byte));
        }
    }

    #[test]
    fn upperhex() {
        for byte in 0x00..=0xFF {
            let char = IsoLatin6Char(byte);
            assert_eq!(format!("{:X}", char), format!("{:X}", byte));
        }
    }

    #[test]
    fn from_self_to_u8() {
        for byte in 0x00..=0xFF {
            let char = IsoLatin6Char(byte);
            assert_eq!(u8::from(char), byte);
        }
    }

    #[test]
    fn from_self_to_char() {
        for (byte, char) in (0xA0..=0xFF).zip(LAST_PART_OF_ISO8859) {
            let isochar = IsoLatin6Char(byte);
            assert_eq!(char::from(isochar), char);
        }
    }

    #[test]
    fn try_from_u8_to_self() {
        for byte in 0x00..=0x7F {
            assert!(IsoLatin6Char::try_from(byte).is_ok(), "0x{byte:x}");
        }

        for byte in 0x80..=0x9F {
            assert_eq!(
                IsoLatin6Char::try_from(byte),
                Err(IsoLatin6CharError::Undefined),
                "{byte:x}"
            );
        }

        for byte in 0xA0..=0xFF {
            assert!(IsoLatin6Char::try_from(byte).is_ok(), "0x{byte:x}");
        }
    }

    #[test]
    fn try_from_char_to_self() {
        for char in '\u{00}'..='\u{7F}' {
            assert!(IsoLatin6Char::try_from(char).is_ok(), "{char}");
        }

        for char in '\u{80}'..='\u{9F}' {
            assert_eq!(
                IsoLatin6Char::try_from(char),
                Err(IsoLatin6CharError::Invalid),
                "{char}"
            );
        }

        for (char, byte) in LAST_PART_OF_ISO8859.into_iter().zip(0xA0..=0xFF) {
            assert_eq!(
                IsoLatin6Char::try_from(char),
                Ok(IsoLatin6Char(byte)),
                "{char} x {}; {byte:x}",
                IsoLatin6Char(byte)
            );
        }
    }
}
