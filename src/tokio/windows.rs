use std::io::Result;
use tokio::process::Command;
use winapi::um::winbase::CREATE_SUSPENDED;

use crate::{winres::*, AsyncCommandGroup, AsyncGroupChild};

#[async_trait::async_trait]
impl AsyncCommandGroup for Command {
	fn group_spawn(&mut self) -> Result<AsyncGroupChild> {
		let (job, completion_port) = job_object()?;
		self.creation_flags(CREATE_SUSPENDED);

		let child = self.spawn()?;
		assign_child(
			child
				.raw_handle()
				.expect("child has exited but it has not even started"),
			job,
		)?;

		Ok(AsyncGroupChild::new(child, job, completion_port))
	}
}
