//! This module implements the types to represent a borrowed string of ISO8859-10 characters.

use std::{fmt, mem, slice::SliceIndex};

use crate::IsoLatin6Char;

/// [IsoLatin6String](crate::IsoLatin6String) slices.
///
/// # Examples
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct IsoLatin6Str {
    bytes: [u8],
}

impl IsoLatin6Str {
    /// Returns the length of the string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// TODO
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns `true` if `self` has a length of zero bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// TODO
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Converts a string slice to a byte slice. To convert the byte slice back
    /// into a string slice, use the [`from_utf8`] function.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// TODO
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { mem::transmute(self) }
    }

    /// Converts a mutable string slice to a mutable byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the content of the slice is valid ISO8859-10
    /// before the borrow ends and the underlying `IsoLatin6Str` is used.
    ///
    /// Use of a `IsoLatin6Str` whose contents are not valid ISO8859-10 is undefined behavior.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// TODO
    ///
    /// Mutability:
    /// TODO
    #[inline(always)]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut()
    }

    /// Converts a string slice to a raw pointer.
    ///
    /// As string slices are a slice of bytes, the raw pointer points to a
    /// [`u8`]. This pointer will be pointing to the first byte of the string
    /// slice.
    ///
    /// The caller must ensure that the returned pointer is never written to.
    /// If you need to mutate the contents of the string slice, use [`as_mut_ptr`].
    ///
    /// [`as_mut_ptr`]: IsoLatin6Str::as_mut_ptr
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// TODO
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    /// Converts a mutable string slice to a raw pointer.
    ///
    /// As string slices are a slice of bytes, the raw pointer points to a
    /// [`u8`]. This pointer will be pointing to the first byte of the string
    /// slice.
    ///
    /// It is your responsibility to make sure that the string slice only gets
    /// modified in a way that it remains valid ISO8859-10.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.bytes.as_mut_ptr()
    }

    /// Returns a subslice of `IsoLatin6Str`.
    ///
    /// This is the non-panicking alternative to indexing the `IsoLatin6Str`. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Examples
    /// TODO
    pub fn get<I>(&self, index: I) -> Option<&Self>
    where I: SliceIndex<[u8], Output = [u8]> {
        // SAFETY: `IsoLatin6Str` is transparently represented the same way as `[u8]`, and therefore
        // safe to transmute between them.
        self.bytes.get(index).map(|s| unsafe { mem::transmute(s) })
    }

    /// Returns a mutable subslice of `IsoLatin6Str`.
    ///
    /// This is the non-panicking alternative to indexing the `IsoLatin6Str`. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Examples
    /// TODO
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut Self>
    where I: SliceIndex<[u8], Output = [u8]> {
        // SAFETY: `IsoLatin6Str` is transparently represented the same way as `[u8]`, and therefore
        // safe to transmute between them.
        self.bytes
            .get_mut(index)
            .map(|s| unsafe { mem::transmute(s) })
    }

    /// Returns an unchecked subslice of `IsoLatin6Str`.
    ///
    /// This is the unchecked alternative to indexing the `IsoLatin6Str`.
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that these preconditions are
    /// satisfied:
    ///
    /// - The starting index must not exceed the ending index
    /// - Indexes must be within bounds of the original slice
    ///
    /// Failing that, the returned string slice may reference invalid memory or
    /// violate the invariants communicated by the `IsoLatin6Str` type.
    ///
    /// # Examples
    ///
    /// TODO
    pub unsafe fn get_unchecked<I>(&self, index: I) -> Option<&Self>
    where I: SliceIndex<[u8], Output = [u8]> {
        // SAFETY: `IsoLatin6Str` is transparently represented the same way as `[u8]`, and therefore
        // safe to transmute between them.
        mem::transmute(self.bytes.get_unchecked(index))
    }

    /// Returns a mutable, unchecked subslice of `IsoLatin6Str`.
    ///
    /// This is the unchecked alternative to indexing the `IsoLatin6Str`.
    ///
    /// # Safety
    ///
    /// Callers of this function are responsible that these preconditions are satisfied:
    ///
    /// - The starting index must not exceed the ending index
    /// - Indexes must be within bounds of the original slice
    ///
    /// Failing that, the returned string slice may reference invalid memory or
    /// violate the invariants communicated by the `IsoLatin6Str` type.
    ///
    /// # Examples
    /// TODO
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> Option<&mut Self>
    where I: SliceIndex<[u8], Output = [u8]> {
        // SAFETY: `IsoLatin6Str` is transparently represented the same way as `[u8]`, and therefore
        // safe to transmute between them.
        mem::transmute(self.bytes.get_unchecked_mut(index))
    }
}

impl fmt::Debug for IsoLatin6Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(r#""{}""#, self))
    }
}

impl fmt::Display for IsoLatin6Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[inline(always)]
        fn write(f: &mut fmt::Formatter<'_>, bytes: &[IsoLatin6Char]) -> Result<usize, fmt::Error> {
            let mut ammount_writed = 0;
            for &ch in bytes {
                f.write_fmt(format_args!("{}", ch))?;
                ammount_writed += 1;
            }
            Ok(ammount_writed)
        }

        #[inline(always)]
        fn write_pads(f: &mut fmt::Formatter, num: usize) -> fmt::Result {
            let fill = f.fill();
            for _ in 0..num {
                f.write_fmt(format_args!("{}", fill))?;
            }
            Ok(())
        }

        // SAFETY: `IsoLatin6Char` has the same representation of `u8`s, and therefore safe to
        // transmute.
        let bytes = unsafe { mem::transmute(&self.bytes) };

        if let Some(align) = f.align() {
            let width = f.width().unwrap_or(0);
            let will_write = self.bytes.len();
            let remaining_pads = width.saturating_sub(will_write);

            match align {
                fmt::Alignment::Left => {
                    let writed = write(f, bytes)?;
                    debug_assert_eq!(will_write, writed);
                    write_pads(f, remaining_pads)?;
                },
                fmt::Alignment::Right => {
                    write_pads(f, remaining_pads)?;
                    let writed = write(f, bytes)?;
                    debug_assert_eq!(will_write, writed);
                },
                fmt::Alignment::Center => {
                    let half = remaining_pads / 2;
                    write_pads(f, half)?;
                    let writed = write(f, bytes)?;
                    debug_assert_eq!(will_write, writed);
                    write_pads(
                        f,
                        if remaining_pads % 2 == 0 {
                            half
                        } else {
                            half + 1
                        },
                    )?;
                },
            }
            Ok(())
        } else {
            write(f, bytes)?;
            Ok(())
        }
    }
}


#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn test_name() {
        todo!()
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;

    #[test]
    fn display() {
        todo!()
    }

    fn debug() {
        todo!()
    }
}
