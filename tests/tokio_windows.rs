#![cfg(all(windows, feature = "with-tokio"))]

use command_group::AsyncCommandGroup;
use std::{io::Result, process::Stdio, time::Duration};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	process::Command,
	time::sleep,
};

const DIE_TIME: Duration = Duration::from_millis(1000);

// each test has a _normal variant that uses the Tokio non-group API for comparison/debugging.

#[tokio::test]
async fn inner_read_stdout_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.spawn()?;

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output).await?;
	}

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[tokio::test]
async fn inner_read_stdout_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.group_spawn()?;

	let mut output = String::new();
	if let Some(mut out) = child.inner().stdout.take() {
		out.read_to_string(&mut output).await?;
	}

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[tokio::test]
async fn into_inner_write_stdin_normal() -> Result<()> {
	let mut child = Command::new("findstr")
		.arg("^")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()?;

	if let Some(mut din) = child.stdin.take() {
		din.write_all(b"hello").await?;
	}

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output).await?;
	}

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[tokio::test]
async fn into_inner_write_stdin_group() -> Result<()> {
	let mut child = Command::new("findstr")
		.arg("^")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.group_spawn()?
		.into_inner();

	if let Some(mut din) = child.stdin.take() {
		din.write_all(b"hello").await?;
	}

	let mut output = String::new();
	if let Some(mut out) = child.stdout.take() {
		out.read_to_string(&mut output).await?;
	}

	assert_eq!(output.as_str(), "hello\r\n");
	Ok(())
}

#[tokio::test]
async fn kill_and_try_wait_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("pause")
		.spawn()?;
	assert!(child.try_wait()?.is_none());
	child.kill().await?;
	sleep(DIE_TIME).await;
	assert!(child.try_wait()?.is_some());
	sleep(DIE_TIME).await;
	assert!(child.try_wait()?.is_some());
	Ok(())
}

#[tokio::test]
async fn kill_and_try_wait_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("pause")
		.group_spawn()?;
	assert!(child.try_wait()?.is_none());
	child.kill()?;
	sleep(DIE_TIME).await;
	assert!(child.try_wait()?.is_some());
	sleep(DIE_TIME).await;
	assert!(child.try_wait()?.is_some());
	Ok(())
}

#[tokio::test]
async fn wait_after_die_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.spawn()?;
	sleep(DIE_TIME).await;

	let status = child.wait().await?;
	assert!(status.success());

	Ok(())
}

#[tokio::test]
async fn wait_after_die_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	sleep(DIE_TIME).await;

	let status = child.wait().await?;
	assert!(status.success());

	Ok(())
}

#[tokio::test]
async fn try_wait_after_die_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.spawn()?;
	sleep(DIE_TIME * 10).await;

	let status = child.try_wait()?;
	assert!(status.is_some());
	assert!(status.unwrap().success());

	Ok(())
}

#[tokio::test]
async fn try_wait_after_die_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	sleep(DIE_TIME * 10).await;

	let status = child.try_wait()?;
	assert!(status.is_some());
	assert!(status.unwrap().success());

	Ok(())
}

#[tokio::test]
async fn wait_normal() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.spawn()?;
	let status = child.wait().await?;
	assert!(status.success());
	let status = child.wait().await?;
	assert!(status.success());
	Ok(())
}

#[tokio::test]
async fn wait_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	let status = child.wait().await?;
	assert!(status.success());
	let status = child.wait().await?;
	assert!(status.success());
	Ok(())
}

#[tokio::test]
async fn wait_with_output_normal() -> Result<()> {
	let child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.spawn()?;

	let output = child.wait_with_output().await?;
	assert!(output.status.success());
	assert_eq!(output.stdout, b"hello\r\n".to_vec());
	assert_eq!(output.stderr, Vec::new());
	Ok(())
}

#[tokio::test]
async fn wait_with_output_group() -> Result<()> {
	let child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.stdout(Stdio::piped())
		.group_spawn()?;

	let output = child.wait_with_output().await?;
	assert!(output.status.success());
	assert_eq!(output.stdout, b"hello\r\n".to_vec());
	assert_eq!(output.stderr, Vec::new());
	Ok(())
}

#[tokio::test]
async fn id_same_as_inner_group() -> Result<()> {
	let mut child = Command::new("powershell.exe")
		.arg("/C")
		.arg("echo hello")
		.group_spawn()?;
	assert_eq!(child.id(), child.inner().id());
	Ok(())
}
