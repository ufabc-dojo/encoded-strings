//! # ISO8859-10 String Library
//!
//! This crate provides string types that are encoded in ISO8859-1, also known as ISO Latin-6.
//!
//! This crate contains the [`IsoLatin6String`] type, the [`ToIsoLatin6String`] trait for
//! converting to strings, and several error types that may result from
//! working with [`IsoLatin6String`]s.

use std::fmt;

/// A ISO8859-1 encoded, growable string.
///
/// # Examples
/// TODO
///
/// # ISO8859-10 or ISO Latin-6
/// TODO
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct IsoLatin6String {
    bytes: Vec<u8>,
}

impl IsoLatin6String {
    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub const fn new() -> Self {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn from_iso8859_10(vec: Vec<u8>) -> Result<Self, FromIso8859_1Error> {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn into_bytes(self) -> Vec<u8> {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub const fn capacity(&self) -> usize {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }

    // You guys got the idea. Try to replicate the String API into the type here.
}

impl fmt::Debug for IsoLatin6String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TIP: Usually for string types the debug implementation is the same as the display implementation but with double quote before and after the text.
        todo!()
    }
}

impl fmt::Display for IsoLatin6String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Docs: TODO
/// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
#[derive(Debug)]
pub struct FromIso8859_1Error {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }

    #[test]
    fn new_works() {
        let s = IsoLatin6String::new();
        assert_eq!(s.capacity(), 0);
    }

    #[test]
    fn with_capacity_works() {
        let s = IsoLatin6String::with_capacity(10);
        assert_eq!(s.capacity(), 10);
    }

    #[test]
    fn from_iso8859_1_works() {
        // Good case
        let s = IsoLatin6String::from_iso8859_10(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.capacity(), 3);
        assert_eq!(s.bytes, vec![0x41, 0x42, 0x43]);

        // Bad case
        // Contains invalid characters
        let res = IsoLatin6String::from_iso8859_10(vec![0x41, 0x42, 0x01, 0x44]);
        assert!(res.is_err()); // FIXME: Ideally, we should have a more specific error type checking here.
    }

    #[test]
    fn into_bytes_works() {
        let s = IsoLatin6String::from_iso8859_10(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.into_bytes(), vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn capacity_works() {
        let s = IsoLatin6String::from_iso8859_10(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.capacity(), 3);
    }

    #[test]
    fn reserve_works() {
        let mut s = IsoLatin6String::from_iso8859_10(vec![0x41, 0x42, 0x43]).unwrap();
        s.reserve(10);
        assert!(s.capacity() >= 13);
    }

    #[test]
    fn reserve_exact_works() {
        let mut s = IsoLatin6String::from_iso8859_10(vec![0x41, 0x42, 0x43]).unwrap();
        s.reserve_exact(10);
        assert_eq!(s.capacity(), 13);
    }
}
