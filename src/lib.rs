//! An extension to [`std::process::Command`] to support process groups on Unix and Windows.
//!
#![cfg_attr(
	unix,
	doc = "On Unix, the [`UnixChildExt`] trait additionally adds support for sending signals to processes and process groups (it’s implemented on _both_ this crate’s [`GroupChild`] and std’s [`Child`](std::process::Child))."
)]
#![doc(html_favicon_url = "https://watchexec.github.io/logo:command-group.svg")]
#![doc(html_logo_url = "https://watchexec.github.io/logo:command-group.svg")]
#![warn(missing_docs)]

mod child;
pub mod stdlib;

#[cfg(windows)]
pub(crate) mod winres;

#[doc(inline)]
pub use child::*;
pub use stdlib::CommandGroup;
