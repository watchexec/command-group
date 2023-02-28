use std::{
	io::Result,
	os::windows::{io::AsRawHandle, process::CommandExt},
	process::Command,
};
use winapi::um::winbase::CREATE_SUSPENDED;

use crate::{builder::GroupBuilder, winres::*, CommandGroup, GroupChild};

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		let (job, completion_port) = job_object(true)?;
		self.creation_flags(CREATE_SUSPENDED);
		let child = self.spawn()?;
		assign_child(child.as_raw_handle(), job)?;

		Ok(GroupChild::new(child, job, completion_port))
	}

	fn group(self) -> GroupBuilder<Command> {
		GroupBuilder::new(self)
	}
}

impl GroupBuilder<std::process::Command> {
	pub fn spawn(&mut self) -> std::io::Result<GroupChild> {
		self.command
			.creation_flags(self.creation_flags | CREATE_SUSPENDED);

		// note: same a as in CommandGroup::group_spawn
		// but without creation_flags(CREATE_SUSPENDED)
		let (job, completion_port) = job_object(self.kill_on_drop)?;
		let child = self.spawn()?;
		assign_child(child.as_raw_handle(), job)?;

		Ok(GroupChild::new(child, job, completion_port))
	}
}
