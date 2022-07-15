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
    /// Creates a new empty `IsoLatin6String`.
    ///
    /// Given that the `IsoLatin6String` is empty, this will not allocate any initial
    /// buffer. While that means that this initial operation is very
    /// inexpensive, it may cause excessive allocation later when you add
    /// data. If you have an idea of how much data the `IsoLatin6String` will hold,
    /// consider the [`with_capacity`] method to prevent excessive
    /// re-allocation.
    ///
    /// [`with_capacity`]: IsoLatin6String::with_capacity
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = IsoLatin6String::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Creates a new empty `IsoLatin6String` with at least the specified capacity.
    ///
    /// `IsoLatin6String`s have an internal buffer to hold their data. The capacity is
    /// the length of that buffer, and can be queried with the [`capacity`]
    /// method. This method creates an empty `IsoLatin6String`, but one with an initial
    /// buffer that can hold at least `capacity` bytes. This is useful when you
    /// may be appending a bunch of data to the `IsoLatin6String`, reducing the number of
    /// reallocations it needs to do.
    ///
    /// [`capacity`]: IsoLatin6String::capacity
    ///
    /// If the given capacity is `0`, no allocation will occur, and this method
    /// is identical to the [`new`] method.
    ///
    /// [`new`]: IsoLatin6String::new
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = IsoLatin6String::with_capacity(10);
    ///
    /// // The String contains no chars, even though it has capacity for more
    /// assert_eq!(s.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// let cap = s.capacity();
    /// for _ in 0..10 {
    ///     s.push('a');
    /// }
    ///
    /// assert_eq!(s.capacity(), cap);
    ///
    /// // ...but this may make the string reallocate
    /// s.push('a');
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    /// Docs: TODO
    /// Tip: You can use the docs of `std::string::String` to get a better idea and inspiration
    pub fn from_iso8859_10(vec: Vec<u8>) -> Result<Self, FromIso8859_1Error> {
        todo!()
    }

    /// Converts a `IsoLatin6String` into a byte vector.
    ///
    /// This consumes the `IsoLatin6String`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = IsoLatin6String::from("hello");
    /// let bytes = s.into_bytes();
    ///
    /// assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
    /// ```
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns this `IsoLatin6String`'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = IsoLatin6String::with_capacity(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    /// Reserves capacity for at least `additional` bytes more than the
    /// current length. The allocator may reserve more space to speculatively
    /// avoid frequent allocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows [`usize`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = IsoLatin6String::new();
    ///
    /// s.reserve(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    ///
    /// This might not actually increase the capacity:
    ///
    /// ```
    /// let mut s = IsoLatin6String::with_capacity(10);
    /// s.push('a');
    /// s.push('b');
    ///
    /// // s now has a length of 2 and a capacity of at least 10
    /// let capacity = s.capacity();
    /// assert_eq!(2, s.len());
    /// assert!(capacity >= 10);
    ///
    /// // Since we already have at least an extra 8 capacity, calling this...
    /// s.reserve(8);
    ///
    /// // ... doesn't actually increase.
    /// assert_eq!(capacity, s.capacity());
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.bytes.reserve(additional)
    }

    /// Reserves the minimum capacity for at least `additional` bytes more than
    /// the current length. Unlike [`reserve`], this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    /// After calling `reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional`. Does nothing if the capacity is already
    /// sufficient.
    ///
    /// [`reserve`]: IsoLatin6String::reserve
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows [`usize`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = IsoLatin6String::new();
    ///
    /// s.reserve_exact(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    ///
    /// This might not actually increase the capacity:
    ///
    /// ```
    /// let mut s = IsoLatin6String::with_capacity(10);
    /// s.push('a');
    /// s.push('b');
    ///
    /// // s now has a length of 2 and a capacity of at least 10
    /// let capacity = s.capacity();
    /// assert_eq!(2, s.len());
    /// assert!(capacity >= 10);
    ///
    /// // Since we already have at least an extra 8 capacity, calling this...
    /// s.reserve_exact(8);
    ///
    /// // ... doesn't actually increase.
    /// assert_eq!(capacity, s.capacity());
    /// ```
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.bytes.reserve_exact(additional)
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
