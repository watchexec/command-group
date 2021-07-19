use std::{
	io::Result,
	process::{Command, ExitStatus, Output},
};

use crate::{CommandGroup, GroupChild};

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		todo!()
	}

	fn group_output(&mut self) -> Result<Output> {
		todo!()
	}

	fn group_status(&mut self) -> Result<ExitStatus> {
		todo!()
	}
}
