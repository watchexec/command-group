#![cfg(windows)]

use command_group::CommandGroup;
use std::{
	io::{Read, Result, Write},
	process::{Command, Stdio},
	thread::sleep,
	time::Duration,
};

const DIE_TIME: Duration = Duration::from_millis(1000);

// each test has a _normal variant that uses the stdlib non-group API for comparison/debugging.

#[test]
fn inner_read_stdout_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.spawn()?;

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output)?;
	}

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[test]
fn inner_read_stdout_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.group_spawn()?;

	let mut output = String::new();
	if let Some(mut out) = child.inner().stdout.take() {
		out.read_to_string(&mut output)?;
	}

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[test]
fn into_inner_write_stdin_normal() -> Result<()> {
	let mut child = Command::new("findstr")
		.arg("^")
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

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[test]
fn into_inner_write_stdin_group() -> Result<()> {
	let mut child = Command::new("findstr")
		.arg("^")
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

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[test]
fn kill_and_try_wait_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("pause")
		.spawn()?;
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
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("pause")
		.group_spawn()?;
	assert!(child.try_wait()?.is_none());
	child.kill()?;
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some());
	sleep(DIE_TIME);
	assert!(child.try_wait()?.is_some());
	Ok(())
}

#[test]
fn wait_after_die_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.spawn()?;
	sleep(DIE_TIME);

	let status = child.wait()?;
	assert!(status.success());

	Ok(())
}

#[test]
fn wait_after_die_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	sleep(DIE_TIME);

	let status = child.wait()?;
	assert!(status.success());

	Ok(())
}

#[test]
fn try_wait_after_die_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.spawn()?;
	sleep(DIE_TIME * 10);

	let status = child.try_wait()?;
	assert!(status.is_some());
	assert!(status.unwrap().success());

	Ok(())
}

#[test]
fn try_wait_after_die_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	sleep(DIE_TIME * 10);

	let status = child.try_wait()?;
	assert!(status.is_some());
	assert!(status.unwrap().success());

	Ok(())
}

#[test]
fn wait_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.spawn()?;
	let status = child.wait()?;
	assert!(status.success());
	let status = child.wait()?;
	assert!(status.success());
	Ok(())
}

#[test]
fn wait_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	let status = child.wait()?;
	assert!(status.success());
	let status = child.wait()?;
	assert!(status.success());
	Ok(())
}

#[test]
fn wait_with_output_normal() -> Result<()> {
	let child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.spawn()?;

	let output = child.wait_with_output()?;
	assert!(output.status.success());
	assert_eq!(output.stdout, b"hello\r\n".to_vec());
	assert_eq!(output.stderr, Vec::new());
	Ok(())
}

#[test]
fn wait_with_output_group() -> Result<()> {
	let child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.group_spawn()?;

	let output = child.wait_with_output()?;
	assert!(output.status.success());
	assert_eq!(output.stdout, b"hello\r\n".to_vec());
	assert_eq!(output.stderr, Vec::new());
	Ok(())
}

#[test]
fn id_same_as_inner_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	assert_eq!(child.id(), child.inner().id());
	Ok(())
}
