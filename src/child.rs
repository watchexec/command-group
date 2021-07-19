use std::{
	io::{Read, Result},
	process::{Child, ExitStatus, Output},
	fmt,
};

#[cfg(unix)]
pub(self) use unix::ChildImp;
#[cfg(windows)]
pub(self) use windows::ChildImp;


#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

/// Representation of a running or exited child process group.
///
/// This wraps the [`Child`] type in the standard library with methods that work
/// with process groups.
///
/// # Examples
///
/// ```should_panic
/// use std::process::Command;
/// use command_group::CommandGroup;
///
/// let mut child = Command::new("/bin/cat")
///                         .arg("file.txt")
///                         .group_spawn()
///                         .expect("failed to execute child");
///
/// let ecode = child.wait()
///                  .expect("failed to wait on child");
///
/// assert!(ecode.success());
/// ```
pub struct GroupChild {
	imp: ChildImp,
}

impl fmt::Debug for GroupChild {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GroupChild").finish()
	}
}

impl GroupChild {
	pub(crate) fn new(inner: Child) -> Self {
		Self {
			imp: ChildImp::new(inner),
		}
	}

	/// Returns the stdlib [`Child`] object.
	///
	/// Note that the inner child may not be in the same state as this output child, due to how
	/// methods like `wait` and `kill` are implemented. It is not recommended to use this method
	/// _after_ using any of the other methods on this struct.
	///
	/// # Examples
	///
	/// Reading from stdout:
	///
	/// ```no_run
	/// use std::io::Read;
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// let mut child = Command::new("ls").group_spawn().expect("ls command didn't start");
	/// let mut output = String::new();
	/// child.inner().stdout.read_to_string(&mut output).expect("failed to read from child");
	/// println!("output: {}", output);
	/// ```
	pub fn inner(&mut self) -> &mut Child {
		self.imp.inner()
	}

	/// Consumes itself and returns the stdlib [`Child`] object.
	///
	/// Note that the inner child may not be in the same state as this output child, due to how
	/// methods like `wait` and `kill` are implemented. It is not recommended to use this method
	/// _after_ using any of the other methods on this struct.
	///
	/// # Examples
	///
	/// Writing to input
	///
	/// ```no_run
	/// use std::io::Write;
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// let mut child = Command::new("cat").group_spawn().expect("cat command didn't start");
	/// child.into_inner().stdin.write_all(b"Woohoo!").expect("failed to write");
	/// ```
	pub fn into_inner(self) -> Child {
		self.imp.into_inner()
	}

	/// Forces the child process group to exit. If the group has already exited, an [`InvalidInput`]
	/// error is returned.
	///
	/// This is equivalent to sending a SIGKILL on Unix platforms.
	///
	/// See [the stdlib documentation][Child::kill] for more.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// let mut command = Command::new("yes");
	/// if let Ok(mut child) = command.group_spawn() {
	///     child.kill().expect("command wasn't running");
	/// } else {
	///     println!("yes command didn't start");
	/// }
	/// ```
	///
	/// [`InvalidInput`]: io::ErrorKind::InvalidInput
	pub fn kill(&mut self) -> Result<()> {
		self.imp.kill()
	}

	/// Returns the OS-assigned process group identifier.
	///
	/// See [the stdlib documentation][Child::id] for more.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// let mut command = Command::new("ls");
	/// if let Ok(child) = command.group_spawn() {
	///     println!("Child group's ID is {}", child.id());
	/// } else {
	///     println!("ls command didn't start");
	/// }
	/// ```
	pub fn id(&self) -> u32 {
		self.imp.id()
	}

	/// Waits for the child group to exit completely, returning the status that
	/// the process leader exited with.
	///
	/// See [the stdlib documentation][Child::wait] for more.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// let mut command = Command::new("ls");
	/// if let Ok(mut child) = command.group_spawn() {
	///     child.wait().expect("command wasn't running");
	///     println!("Child has finished its execution!");
	/// } else {
	///     println!("ls command didn't start");
	/// }
	/// ```
	pub fn wait(&mut self) -> Result<ExitStatus> {
		drop(self.imp.take_stdin());
		self.imp.wait()
	}

	/// Attempts to collect the exit status of the child if it has already
	/// exited.
	///
	/// See [the stdlib documentation][Child::try_wait] for more.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```no_run
	/// use std::process::Command;
	/// use command_group::CommandGroup;
	///
	/// let mut child = Command::new("ls").group_spawn().unwrap();
	///
	/// match child.try_wait() {
	///     Ok(Some(status)) => println!("exited with: {}", status),
	///     Ok(None) => {
	///         println!("status not ready yet, let's really wait");
	///         let res = child.wait();
	///         println!("result: {:?}", res);
	///     }
	///     Err(e) => println!("error attempting to wait: {}", e),
	/// }
	/// ```
	pub fn try_wait(&mut self) -> Result<Option<ExitStatus>> {
		self.imp.try_wait()
	}

	/// Simultaneously waits for the child to exit and collect all remaining
	/// output on the stdout/stderr handles, returning an `Output`
	/// instance.
	///
	/// See [the stdlib documentation][Child::wait_with_output] for more.
	///
	/// # Examples
	///
	/// ```should_panic
	/// use std::process::{Command, Stdio};
	/// use command_group::CommandGroup;
	///
	/// let child = Command::new("/bin/cat")
	///     .arg("file.txt")
	///     .stdout(Stdio::piped())
	///     .group_spawn()
	///     .expect("failed to execute child");
	///
	/// let output = child
	///     .wait_with_output()
	///     .expect("failed to wait on child");
	///
	/// assert!(output.status.success());
	/// ```
	///
	pub fn wait_with_output(mut self) -> Result<Output> {
		drop(self.imp.take_stdin());

        let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
        match (self.imp.take_stdout(), self.imp.take_stderr()) {
            (None, None) => {}
            (Some(mut out), None) => {
                out.read_to_end(&mut stdout)?;
            }
            (None, Some(mut err)) => {
                err.read_to_end(&mut stderr)?;
            }
            (Some(out), Some(err)) => {
                let res = ChildImp::read_both(out, &mut stdout, err, &mut stderr);
                res.unwrap();
            }
        }

        let status = self.imp.wait()?;
        Ok(Output { status, stdout, stderr })
	}
}
