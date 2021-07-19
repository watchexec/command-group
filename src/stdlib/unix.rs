use std::{
	io::{Error, Result},
	os::unix::process::CommandExt,
	process::{Command, ExitStatus, Output},
};

use crate::{CommandGroup, GroupChild};
use nix::unistd::setsid;

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		unsafe {
			self.pre_exec(|| setsid().map_err(Error::from).map(|_| ()));
		}

		self.spawn().map(GroupChild::new)
	}

	fn group_output(&mut self) -> Result<Output> {
		self.group_spawn()
			.and_then(|child| child.wait_with_output())
	}

	fn group_status(&mut self) -> Result<ExitStatus> {
		self.group_spawn().and_then(|mut child| child.wait())
	}
}
