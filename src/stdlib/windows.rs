use std::{
	io::Result,
	os::windows::{io::AsRawHandle, process::CommandExt},
	process::Command,
};
use winapi::um::winbase::CREATE_SUSPENDED;

use crate::{winres::*, CommandGroup, GroupChild};

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		let (job, completion_port) = job_object()?;
		self.creation_flags(CREATE_SUSPENDED);
		let child = self.spawn()?;
		assign_child(child.as_raw_handle(), job)?;

		Ok(GroupChild::new(child, job, completion_port))
	}
}
