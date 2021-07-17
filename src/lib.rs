//! An extension to [std::process::Command] to support process groups on Unix and Windows.
//!
//! Also supports async-std with the `async-std` feature.

mod child;
pub mod stdlib;

pub use child::GroupChild;
pub use stdlib::CommandGroup;
