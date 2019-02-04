#![cfg_attr(feature = "nightly", feature(try_from))]

//! This crate provides unsigned floats by defining two wrappers - a UF32 and UF64.
//! The type is equivalent to storing a positive number of the underlying type (f32 or f64)
//! and no optimisations have been carried out to use the sign bit to store another bit of
//! the data.
//!
//! ## Installation
//!
//! To use this as a dependency, add it to your Cargo.toml file:
//!
//! ```toml
//! ufo = "0.1"
//! ```
//!
//! If you need serialization, enable the serde feature
//! ```toml
//! ufo = { version = "0.1", features = ["serde"] }
//! ```
//!
//! There is also a nightly feature that takes advantage of the TryFrom/TryInto traits
//!
//! ## Usage
//!
//! ```rust
//! use ufo::Uf32;
//!
//! fn main() {
//!     let a = Uf32::try_new(1.0).expect("invalid number");
//!     let b = Uf32::try_new(2.0).expect("invalid number");
//!     assert_eq!(a + b, Uf32::try_new(3.0).expect("invalid number"))
//! }
//! ```
//!

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "nightly")]
use std::convert::TryFrom;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[doc(hidden)]
pub mod math;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
/// Uf32 represents an unsigned f32. You cannot create a new value directly
/// and have to use the `try_new()` method or by calling `try_into` on a f32,
/// where a check is made to ensure that the value is positive.
///
/// Uf32 implements most of the common traits that are implemented on f32 so they
/// can be used directly. This includes addition, subtraction, multiplication,
/// division along with the assignment functions (+=, *=, etc).
pub struct Uf32(f32);

impl Uf32 {
    /// Creates a new Uf32. The function returns a `InvalidNumberError` if a negative
    /// or other invalid numbers are passed to the function. This function should be used
    /// since it is not possible to create a Uf32 directly (i.e. `Uf32(2.0)` will fail
    /// to compile.
    pub fn try_new(value: f32) -> Result<Uf32, InvalidNumberError> {
        if value > 0f32 {
            Ok(Uf32(value))
        } else {
            Err(InvalidNumberError(value))
        }
    }

}
#[cfg(not(feature="nightly"))]
/// Polyfill trait that mimics the TryInto trait on stable till it is stabilized.
pub trait TryInto : Sized {
    fn try_into(self) -> Result<Uf32, InvalidNumberError>;

}

#[cfg(not(feature="nightly"))]
impl TryInto for f32 {
    fn try_into(self) -> Result<Uf32, InvalidNumberError> {
        Uf32::try_new(self)
    }
}

impl Display for Uf32 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", &self.0)
    }
}

impl Deref for Uf32 {
    type Target = f32;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Uf32 {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

#[cfg(feature="nightly")]
impl TryFrom<f32> for Uf32 {
    type Error = InvalidNumberError;

    fn try_from(value: f32) -> Result<Self, <Self as TryFrom<f32>>::Error> {
        Uf32::try_new(value)
    }
}

/// An error that represents either a negative or an invalid number.
#[derive(Clone, Copy, Debug)]
pub struct InvalidNumberError(f32);

impl Display for InvalidNumberError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} is invalid for a Uf32.",  &self)
    }
}

impl Error for InvalidNumberError {}

#[cfg(tests)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let a = Uf32::try_new(1.0).unwrap();
        let b = Uf32::try_new(2.0).unwrap();

        assert_eq!(a + &b, Uf32::try_new(3.0).unwrap());
    }
}

