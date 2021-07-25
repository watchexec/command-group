//! An extension to [`std::process::Command`] to support process groups on Unix and Windows.
//!
#![cfg_attr(
	unix,
	doc = "On Unix, the [`UnixChildExt`] trait additionally adds support for sending signals to processes and process groups (it’s implemented on _both_ this crate’s [`GroupChild`] and std’s [`Child`](std::process::Child))."
)]
#![doc(html_favicon_url = "https://watchexec.github.io/logo:command-group.svg")]
#![doc(html_logo_url = "https://watchexec.github.io/logo:command-group.svg")]
#![warn(missing_docs)]

pub mod stdlib;

#[cfg(unix)]
mod unix_ext;

#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(windows)]
pub(crate) mod winres;

#[cfg(unix)]
#[doc(inline)]
pub use crate::unix_ext::UnixChildExt;
#[cfg(unix)]
#[doc(no_inline)]
pub use nix::sys::signal::Signal;

#[doc(inline)]
pub use crate::stdlib::child::GroupChild;
pub use crate::stdlib::CommandGroup;

#[cfg(feature = "tokio")]
#[doc(inline)]
pub use crate::tokio::AsyncCommandGroup;
