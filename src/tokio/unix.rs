use std::io::{Error, Result};

use crate::builder::CommandGroupBuilder;
use crate::{AsyncCommandGroup, AsyncGroupChild};
use nix::unistd::setsid;
use tokio::process::Command;

#[async_trait::async_trait]
impl AsyncCommandGroup for Command {
	fn group_spawn(&mut self) -> Result<AsyncGroupChild> {
		unsafe {
			self.pre_exec(|| setsid().map_err(Error::from).map(|_| ()));
		}

		self.spawn().map(AsyncGroupChild::new)
	}

	fn group(&mut self) -> crate::builder::CommandGroupBuilder<tokio::process::Command> {
		crate::builder::CommandGroupBuilder::new(self)
	}
}

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
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// Command::new("ls")
	///         .group()
	/// 		.spawn()
	///         .expect("ls command failed to start");
	/// ```
	pub fn spawn(&mut self) -> std::io::Result<AsyncGroupChild> {
		self.command.group_spawn()
	}
}
