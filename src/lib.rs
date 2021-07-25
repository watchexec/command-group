//! An extension to [`std::process::Command`] to support process groups on Unix and Windows.
#![cfg_attr(
	feature = "tokio",
	doc = "With Tokio, the [`AsyncCommandGroup`] trait extends [`tokio::process::Command`](::tokio::process::Command)."
)]
#![doc = "\n"]
#![cfg_attr(
	unix,
	doc = "On Unix, the [`UnixChildExt`] trait additionally provides"
)]
#![cfg_attr(
	unix,
	doc = "support for sending signals to processes and process groups (it’s implemented on this crate’s [`GroupChild`],"
)]
#![cfg_attr(
	all(unix, feature = "tokio"),
	doc = "[`AsyncGroupChild`], Tokio’s [`Child`](::tokio::process::Child)"
)]
#![cfg_attr(unix, doc = "and std’s [`Child`](std::process::Child)).")]
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
pub use crate::tokio::child::AsyncGroupChild;
#[cfg(feature = "tokio")]
pub use crate::tokio::AsyncCommandGroup;
