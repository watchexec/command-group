use std::{
	io::{Error, Result},
	os::unix::process::CommandExt,
	process::Command,
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
}
