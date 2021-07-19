//! An extension to [std::process::Command] to support process groups on Unix and Windows.
//!
//! Also supports async-std with the `async-std` feature.
//!
//! On Unix, the [`UnixChildExt`] trait additionally adds support for sending signals to processes
//! and process groups (it’s implemented on _both_ this crate’s [`GroupChild`] and std’s [`Child`]).

#![warn(missing_docs)]

mod child;
pub mod stdlib;

#[cfg(windows)]
pub(crate) mod winres;

#[doc(inline)]
pub use child::*;
pub use stdlib::CommandGroup;
