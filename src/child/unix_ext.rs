use std::{
	io::{Error, Result},
	process::Child,
	convert::TryInto,
};

use nix::{
	sys::{
        signal::{Signal, kill},
    },
	unistd::Pid,
};

/// Unix-specific extensions to process [`Child`]ren.
pub trait UnixChildExt {
	/// Sends a signal to the child process. If the process has already exited, an [`InvalidInput`]
	/// error is returned.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::UnixChildExt;
	///
	/// let mut command = Command::new("yes");
	/// if let Ok(mut child) = command.spawn() {
	///     child.signal(Signal::SIGTERM).expect("command wasn't running");
	/// } else {
	///     println!("yes command didn't start");
	/// }
	/// ```
	///
	/// With a process group:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::{CommandGroup, UnixChildExt};
	///
	/// let mut command = Command::new("yes");
	/// if let Ok(mut child) = command.group_spawn() {
	///     child.signal(Signal::SIGTERM).expect("command wasn't running");
	/// } else {
	///     println!("yes command didn't start");
	/// }
	/// ```
	///
	/// [`InvalidInput`]: io::ErrorKind::InvalidInput
	fn signal(&mut self, sig: Signal) -> Result<()>;
}

impl UnixChildExt for super::GroupChild {
	fn signal(&mut self, sig: Signal) -> Result<()> {
		self.imp.signal_imp(sig)
	}
}

impl UnixChildExt for Child {
	fn signal(&mut self, sig: Signal) -> Result<()> {
		let pid = Pid::from_raw(self.id().try_into().expect("Command PID > i32::MAX"));
		kill(pid, sig).map_err(Error::from)
	}
}
