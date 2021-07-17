use std::{
	io::Result,
	process::{Child, ExitStatus, Output},
};

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
#[derive(Debug)]
pub struct GroupChild {
	inner: Child,
}

impl GroupChild {
	/// Returns the stdlib [`Child`] object.
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
		&mut self.inner
	}

	/// Consumes itself and returns the stdlib [`Child`] object.
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
		self.inner
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!()
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
		todo!("{:?}", self)
	}
}
