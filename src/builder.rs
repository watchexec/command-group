/// GroupBuilder is a builder for a group of processes.
///
/// It is created via the `group` method on [`Command`](std::process::Command) or
/// [`AsyncCommand`](tokio::process::Command).
pub struct GroupBuilder<'a, T> {
	pub(crate) command: &'a mut T,
	pub(crate) kill_on_drop: bool,
	pub(crate) creation_flags: u32,
}

impl<'a, T> GroupBuilder<'a, T> {
	pub(crate) fn new(command: &'a mut T) -> Self {
		Self {
			command,
			kill_on_drop: false,
			creation_flags: 0,
		}
	}

	pub fn creation_flags(mut self, creation_flags: u32) -> Self {
		self.creation_flags = creation_flags;
		self
	}
}
