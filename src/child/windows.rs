use std::{
	io::Result,
	process::{Child, ChildStderr, ChildStdin, ChildStdout, ExitStatus, Output},
};

pub(super) struct ChildImp {
	inner: Child,
}

impl ChildImp {
	pub fn new(inner: Child) -> Self {
		Self { inner }
	}

	pub(super) fn take_stdin(&mut self) -> Option<ChildStdin> {
		self.inner.stdin.take()
	}

	pub(super) fn take_stdout(&mut self) -> Option<ChildStdout> {
		self.inner.stdout.take()
	}

	pub(super) fn take_stderr(&mut self) -> Option<ChildStderr> {
		self.inner.stderr.take()
	}

	pub fn inner(&mut self) -> &mut Child {
		&mut self.inner
	}

	pub fn into_inner(self) -> Child {
		self.inner
	}

	pub fn kill(&mut self) -> Result<()> {
		todo!()
	}

	pub fn id(&self) -> u32 {
		self.inner.id()
	}

	pub fn wait(&mut self) -> Result<ExitStatus> {
		todo!()
	}

	pub fn try_wait(&mut self) -> Result<Option<ExitStatus>> {
		todo!()
	}

	pub(super) fn read_both(
		mut out_r: ChildStdout,
		out_v: &mut Vec<u8>,
		mut err_r: ChildStderr,
		err_v: &mut Vec<u8>,
	) -> Result<()> {
		todo!()
	}
}
