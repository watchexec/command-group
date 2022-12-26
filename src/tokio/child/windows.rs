use std::{io::Result, mem, process::ExitStatus};
use tokio::{
	process::{Child, ChildStderr, ChildStdin, ChildStdout},
	task::spawn_blocking,
};
use winapi::{
	shared::{
		basetsd::ULONG_PTR,
		minwindef::{DWORD, FALSE},
	},
	um::{
		handleapi::CloseHandle, ioapiset::GetQueuedCompletionStatus, jobapi2::TerminateJobObject,
		minwinbase::OVERLAPPED, winbase::INFINITE, winnt::HANDLE,
	},
};

use crate::winres::*;

pub(super) struct ChildImp {
	inner: Child,
	handles: JobPort,
}

impl ChildImp {
	pub fn new(inner: Child, job: HANDLE, completion_port: HANDLE) -> Self {
		Self {
			inner,
			handles: JobPort {
				job,
				completion_port,
			},
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
		let its = mem::ManuallyDrop::new(self.handles);

		// manually drop the completion port
		unsafe { CloseHandle(its.completion_port) };
		// we leave the job handle unclosed, otherwise the Child is useless
		// (as closing it will terminate the job)

		self.inner
	}

	pub fn kill(&mut self) -> Result<()> {
		res_bool(unsafe { TerminateJobObject(self.handles.job, 1) })
	}

	pub fn id(&self) -> Option<u32> {
		self.inner.id()
	}

	fn wait_imp(handles: JobPort, timeout: DWORD) -> Result<()> {
		let mut code: DWORD = 0;
		let mut key: ULONG_PTR = 0;
		let mut overlapped = mem::MaybeUninit::<OVERLAPPED>::uninit();
		let mut lp_overlapped = overlapped.as_mut_ptr();

		let result = unsafe {
			GetQueuedCompletionStatus(
				handles.completion_port,
				&mut code,
				&mut key,
				&mut lp_overlapped,
				timeout,
			)
		};

		// ignore timing out errors unless the timeout was specified to INFINITE
		// https://docs.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-getqueuedcompletionstatus
		if timeout != INFINITE && result == FALSE && lp_overlapped.is_null() {
			return Ok(());
		}

		res_bool(result)?;

		// don't drop them
		mem::forget(handles);

		Ok(())
	}

	pub async fn wait(&mut self) -> Result<ExitStatus> {
		let handles = self.handles.clone();
		spawn_blocking(|| Self::wait_imp(handles, INFINITE)).await??;
		self.inner.wait().await
	}

	pub fn try_wait(&mut self) -> Result<Option<ExitStatus>> {
		Self::wait_imp(self.handles.clone(), 0)?;
		self.inner.try_wait()
	}
}
