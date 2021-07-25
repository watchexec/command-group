use std::{
	io::Result,
	mem,
	os::windows::io::AsRawHandle,
	process::{Child as StdChild, ExitStatus},
};
use tokio::{
	process::{Child, ChildStderr, ChildStdin, ChildStdout, Command},
	sync::oneshot,
};
use winapi::um::{winbase::CREATE_SUSPENDED, winnt::HANDLE};

use crate::{winres::*, AsyncCommandGroup, AsyncGroupChild};

#[async_trait::async_trait]
impl AsyncCommandGroup for Command {
	fn group_spawn(&mut self) -> Result<AsyncGroupChild> {
		let (job, completion_port) = job_object()?;
		self.creation_flags(CREATE_SUSPENDED);
		let child = self.spawn()?;

		// this is incredibly unsafe and also relies on:
		// - tokio internals staying the same
		// - rust layout optimiser not fucking us
		// if https://github.com/tokio-rs/tokio/issues/3987 gets done, use it instead!

		// we could use transmute_copy here, but I want to rely on the compiler telling me if the
		// types change size to get at least maybe a smidge of a chance to catch internals changing.
		let uninternal_child: TokioChild = unsafe { mem::transmute(child) };
		let handle =
			if let TokioChild {
				child:
					FusedChild::Child(ChildDropGuard {
						inner: TokioImpChild { ref child, .. },
						..
					}),
				..
			} = uninternal_child
			{
				child.as_raw_handle()
			} else {
				panic!("child has exited but it has not even started // OR something unsafe is going on");
			};
		let child: Child = unsafe { mem::transmute(uninternal_child) };

		assign_child(handle, job)?;

		Ok(AsyncGroupChild::new(child, job, completion_port))
	}
}

struct TokioChild {
	child: FusedChild,
	_stdin: Option<ChildStdin>,
	_stdout: Option<ChildStdout>,
	_stderr: Option<ChildStderr>,
}

#[allow(dead_code)]
enum FusedChild {
	Child(ChildDropGuard<TokioImpChild>),
	Done(ExitStatus),
}

struct ChildDropGuard<T> {
	inner: T,
	_kill_on_drop: bool,
}

struct TokioImpChild {
	child: StdChild,
	_waiting: Option<Waiting>,
}

// the only reason why we need the sync feature:
struct Waiting {
	_rx: oneshot::Receiver<()>,
	_wait_object: HANDLE,
	_tx: *mut Option<oneshot::Sender<()>>,
}
