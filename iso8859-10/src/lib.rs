//! # ISO8859-10 String Library
//!
//! This crate provides string types that are encoded in ISO8859-1, also known as ISO Latin-6.
//!
//! This crate contains the [`IsoLatin6String`] type, the [`ToIsoLatin6String`] trait for
//! converting to strings, and several error types that may result from
//! working with [`IsoLatin6String`]s.

pub mod char;
mod map;
pub mod str;
pub mod string;

pub use crate::{char::IsoLatin6Char, str::IsoLatin6Str, string::IsoLatin6String};
