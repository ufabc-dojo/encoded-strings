//! # ISO8859-1 String Library
//!
//! This crate provides string types that are encoded in ISO8859-1.

use std::fmt;

/// A ISO8859-1 encoded, growable string.
///
/// # Examples
/// TODO
///
/// # ISO8859-1
/// TODO
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Iso8859_1String {
    bytes: Vec<u8>,
}

impl Iso8859_1String {
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
    pub fn from_iso8859_1(vec: Vec<u8>) -> Result<Self, FromIso8859_1Error> {
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

impl fmt::Debug for Iso8859_1String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TIP: Usually for string types the debug implementation is the same as the display implementation but with double quote before and after the text.
        todo!()
    }
}

impl fmt::Display for Iso8859_1String {
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
        let s = Iso8859_1String::new();
        assert_eq!(s.capacity(), 0);
    }

    #[test]
    fn with_capacity_works() {
        let s = Iso8859_1String::with_capacity(10);
        assert_eq!(s.capacity(), 10);
    }

    #[test]
    fn from_utf8859_1_works() {
        // Good case
        let s = Iso8859_1String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.capacity(), 3);
        assert_eq!(s.bytes, vec![0x41, 0x42, 0x43]);

        // Bad case
        // Contains invalid characters
        let res = Iso8859_1String::from_iso8859_1(vec![0x41, 0x42, 0x00, 0x44]);
        assert!(res.is_err()); // FIXME: Ideally, we should have a more specific error type checking here.
    }

    #[test]
    fn into_bytes_works() {
        let s = Iso8859_1String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.into_bytes(), vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn capacity_works() {
        let s = Iso8859_1String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        assert_eq!(s.capacity(), 3);
    }

    #[test]
    fn reserve_works() {
        let mut s = Iso8859_1String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        s.reserve(10);
        assert!(s.capacity() >= 13);
    }

    #[test]
    fn reserve_exact_works() {
        let mut s = Iso8859_1String::from_iso8859_1(vec![0x41, 0x42, 0x43]).unwrap();
        s.reserve_exact(10);
        assert_eq!(s.capacity(), 13);
    }
}
