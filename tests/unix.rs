#![cfg(unix)]

use command_group::{CommandGroup, Signal, UnixChildExt};
use std::{
	io::{Read, Result, Write},
	os::unix::process::ExitStatusExt,
	process::{Command, Stdio},
	thread::sleep,
	time::Duration,
};

const DIE_TIME: Duration = Duration::from_millis(100);

// each test has a _normal variant that uses the stdlib non-group API for comparison/debugging.

#[test]
fn inner_read_stdout_normal() -> Result<()> {
	let mut child = Command::new("echo")
		.arg("hello")
		.stdout(Stdio::piped())
		.spawn()?;

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output)?;
	}

	assert_eq!(output.as_str(), "hello\n");
	Ok(())
}

#[test]
fn inner_read_stdout_group() -> Result<()> {
	let mut child = Command::new("echo")
		.arg("hello")
		.stdout(Stdio::piped())
		.group_spawn()?;

	let mut output = String::new();
	if let Some(mut out) = child.inner().stdout.take() {
		out.read_to_string(&mut output)?;
	}

	assert_eq!(output.as_str(), "hello\n");
	Ok(())
}

#[test]
fn into_inner_write_stdin_normal() -> Result<()> {
	let mut child = Command::new("cat")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	if let Some(mut din) = child.stdin.take() {
		din.write_all(b"hello")?;
	}

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output)?;
	}

	assert_eq!(output.as_str(), "hello");
	Ok(())
}

#[test]
fn into_inner_write_stdin_group() -> Result<()> {
	let mut child = Command::new("cat")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.group_spawn()?
		.into_inner();

	if let Some(mut din) = child.stdin.take() {
		din.write_all(b"hello")?;
	}

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output)?;
	}

	assert_eq!(output.as_str(), "hello");
	Ok(())
}

#[test]
fn kill_and_try_wait_normal() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).spawn()?;
	assert!(child.try_wait()?.is_none());
	child.kill()?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some());
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some());
	Ok(())
}

#[test]
fn kill_and_try_wait_group() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).group_spawn()?;
	assert!(child.try_wait()?.is_none());
	child.kill()?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some());
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some());
	Ok(())
}

#[test]
fn try_wait_twice_after_sigterm_normal() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).spawn()?;
	assert!(child.try_wait()?.is_none(), "pre try_wait");
	child.signal(Signal::SIGTERM)?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some(), "first try_wait");
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some(), "second try_wait");
	Ok(())
}

#[test]
fn try_wait_twice_after_sigterm_group() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).group_spawn()?;
	assert!(child.try_wait()?.is_none(), "pre try_wait");
	child.signal(Signal::SIGTERM)?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some(), "first try_wait");
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some(), "second try_wait");
	Ok(())
}

#[test]
fn wait_twice_after_sigterm_normal() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).spawn()?;
	assert!(child.try_wait()?.is_none(), "pre try_wait");
	child.signal(Signal::SIGTERM)?;
	let status = child.wait()?;
	assert_eq!(
		status.signal(),
		Some(Signal::SIGTERM as i32),
		"first wait status"
	);
	let status = child.wait()?;
	assert_eq!(
		status.signal(),
		Some(Signal::SIGTERM as i32),
		"second wait status"
	);
	Ok(())
}

#[test]
fn wait_twice_after_sigterm_group() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).group_spawn()?;
	assert!(child.try_wait()?.is_none(), "pre try_wait");
	child.signal(Signal::SIGTERM)?;
	let status = child.wait()?;
	assert_eq!(
		status.signal(),
		Some(Signal::SIGTERM as i32),
		"first wait status"
	);
	let status = child.wait()?;
	assert_eq!(
		status.signal(),
		Some(Signal::SIGTERM as i32),
		"second wait status"
	);
	Ok(())
}

#[test]
fn wait_after_die_normal() -> Result<()> {
	let mut child = Command::new("echo").stdout(Stdio::null()).spawn()?;
	sleep(DIE_TIME);

	let status = child.wait()?;
	assert!(status.success());

	Ok(())
}

#[test]
fn wait_after_die_group() -> Result<()> {
	let mut child = Command::new("echo").stdout(Stdio::null()).group_spawn()?;
	sleep(DIE_TIME);

	let status = child.wait()?;
	assert!(status.success());

	Ok(())
}

#[test]
fn try_wait_after_die_normal() -> Result<()> {
	let mut child = Command::new("echo").stdout(Stdio::null()).spawn()?;
	sleep(DIE_TIME);

	let status = child.try_wait()?;
	assert!(status.is_some());
	assert!(status.unwrap().success());

	Ok(())
}

#[test]
fn try_wait_after_die_group() -> Result<()> {
	let mut child = Command::new("echo").stdout(Stdio::null()).group_spawn()?;
	sleep(DIE_TIME);

	let status = child.try_wait()?;
	assert!(status.is_some());
	assert!(status.unwrap().success());

	Ok(())
}

#[test]
fn wait_normal() -> Result<()> {
	let mut command = Command::new("echo");
	let mut child = command.spawn()?;
	let status = child.wait()?;
	assert!(status.success());
	let status = child.wait()?;
	assert!(status.success());
	Ok(())
}

#[test]
fn wait_group() -> Result<()> {
	let mut command = Command::new("echo");
	let mut child = command.group_spawn()?;
	let status = child.wait()?;
	assert!(status.success());
	let status = child.wait()?;
	assert!(status.success());
	Ok(())
}

#[test]
fn wait_with_output_normal() -> Result<()> {
	let child = Command::new("echo")
		.arg("hello")
		.stdout(Stdio::piped())
		.spawn()?;

	let output = child.wait_with_output()?;
	assert!(output.status.success());
	assert_eq!(output.stdout, b"hello\n".to_vec());
	assert_eq!(output.stderr, Vec::new());
	Ok(())
}

#[test]
fn wait_with_output_group() -> Result<()> {
	let child = Command::new("echo")
		.arg("hello")
		.stdout(Stdio::piped())
		.group_spawn()?;

	let output = child.wait_with_output()?;
	assert!(output.status.success());
	assert_eq!(output.stdout, b"hello\n".to_vec());
	assert_eq!(output.stderr, Vec::new());
	Ok(())
}

#[test]
fn id_same_as_inner_group() -> Result<()> {
	let mut command = Command::new("echo");
	let mut child = command.group_spawn()?;
	assert_eq!(child.id(), child.inner().id());
	Ok(())
}

#[test]
fn signal_normal() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).spawn()?;

	child.signal(Signal::SIGCONT)?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_none(), "not exited with sigcont");

	child.signal(Signal::SIGTERM)?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some(), "exited with sigterm");

	Ok(())
}

#[test]
fn signal_group() -> Result<()> {
	let mut child = Command::new("yes").stdout(Stdio::null()).group_spawn()?;

	child.signal(Signal::SIGCONT)?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_none(), "not exited with sigcont");

	child.signal(Signal::SIGTERM)?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some(), "exited with sigterm");

	Ok(())
}
