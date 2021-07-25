use std::io::{Error, Result};

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
}
