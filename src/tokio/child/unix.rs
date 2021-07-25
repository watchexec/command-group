use std::{
	convert::TryInto,
	io::{Error, ErrorKind, Result},
	os::unix::process::ExitStatusExt,
	process::ExitStatus,
};

use nix::{
	errno::Errno,
	libc,
	sys::{
		signal::{killpg, Signal},
		wait::WaitPidFlag,
	},
	unistd::Pid,
};
use tokio::{
	process::{Child, ChildStderr, ChildStdin, ChildStdout},
	task::spawn_blocking,
};

pub(super) struct ChildImp {
	pgid: Pid,
	inner: Child,
}

impl ChildImp {
	pub(super) fn new(inner: Child) -> Self {
		let pid = inner
			.id()
			.expect("Command was reaped before we could read its PID")
			.try_into()
			.expect("Command PID > i32::MAX");
		Self {
			pgid: Pid::from_raw(pid),
			inner,
		}
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

	pub(super) fn signal_imp(&mut self, sig: Signal) -> Result<()> {
		killpg(self.pgid, sig).map_err(Error::from)
	}

	pub fn kill(&mut self) -> Result<()> {
		self.signal_imp(Signal::SIGKILL)
	}

	pub fn id(&self) -> Option<u32> {
		self.inner.id()
	}

	fn wait_imp(pgid: i32, flag: WaitPidFlag) -> Result<Option<ExitStatus>> {
		// we can't use the safe wrapper directly because it doesn't return the raw status, and we
		// need it to convert to the std's ExitStatus.
		let mut status: i32 = 0;
		match unsafe { libc::waitpid(-pgid, &mut status as *mut libc::c_int, flag.bits()) } {
			0 => Ok(None),
			res => Errno::result(res)
				.map_err(Error::from)
				.map(|_| Some(ExitStatus::from_raw(status))),
		}
	}

	pub async fn wait(&mut self) -> Result<ExitStatus> {
		let pgid = self.pgid.as_raw();
		spawn_blocking(move || Self::wait_imp(pgid, WaitPidFlag::empty()))
			.await?
			.transpose()
			.unwrap_or_else(|| {
				Err(Error::new(
					ErrorKind::Other,
					"blocking waitpid returned pid=0",
				))
			})
	}

	pub fn try_wait(&mut self) -> Result<Option<ExitStatus>> {
		Self::wait_imp(self.pgid.as_raw(), WaitPidFlag::WNOHANG)
	}
}

impl crate::UnixChildExt for ChildImp {
	fn signal(&mut self, sig: Signal) -> Result<()> {
		self.signal_imp(sig)
	}
}
