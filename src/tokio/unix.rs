use std::io::Error;

use crate::builder::CommandGroupBuilder;
use crate::AsyncGroupChild;
use nix::unistd::setsid;

impl CommandGroupBuilder<'_, tokio::process::Command> {
	/// Executes the command as a child process group, returning a handle to it.
	///
	/// By default, stdin, stdout and stderr are inherited from the parent.
	///
	/// On Windows, this creates a job object instead of a POSIX process group.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use tokio::process::Command;
	/// use command_group::CommandGroup;
	///
	/// Command::new("ls")
	///         .group()
	/// 		.spawn()
	///         .expect("ls command failed to start");
	/// ```
	pub fn spawn(&mut self) -> std::io::Result<AsyncGroupChild> {
		unsafe {
			self.command
				.pre_exec(|| setsid().map_err(Error::from).map(|_| ()));
		}

		self.command.spawn().map(AsyncGroupChild::new)
	}
}
