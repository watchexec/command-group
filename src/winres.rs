use std::{
	convert::TryInto,
	io::{Error, Result},
	mem,
	os::windows::io::RawHandle,
	ptr,
};
use winapi::{
	shared::minwindef::{BOOL, DWORD, FALSE, LPVOID},
	um::{
		handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
		ioapiset::CreateIoCompletionPort,
		jobapi2::{AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject},
		processthreadsapi::{GetProcessId, OpenThread, ResumeThread},
		tlhelp32::{
			CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
		},
		winnt::{
			JobObjectAssociateCompletionPortInformation, JobObjectExtendedLimitInformation, HANDLE,
			JOBOBJECT_ASSOCIATE_COMPLETION_PORT, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
			JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
		},
	},
};

#[derive(Clone)]
pub(crate) struct JobPort {
	pub job: HANDLE,
	pub completion_port: HANDLE,
}

impl Drop for JobPort {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.job) };
		unsafe { CloseHandle(self.completion_port) };
	}
}

unsafe impl Send for JobPort {}
unsafe impl Sync for JobPort {}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct ThreadSafeRawHandle(pub HANDLE);

unsafe impl Send for ThreadSafeRawHandle {}
unsafe impl Sync for ThreadSafeRawHandle {}

pub(crate) fn res_null(handle: HANDLE) -> Result<HANDLE> {
	if handle.is_null() {
		Err(Error::last_os_error())
	} else {
		Ok(handle)
	}
}

pub(crate) fn res_bool(ret: BOOL) -> Result<()> {
	if ret == FALSE {
		Err(Error::last_os_error())
	} else {
		Ok(())
	}
}

pub(crate) fn res_neg(ret: DWORD) -> Result<DWORD> {
	if ret == DWORD::MAX {
		Err(Error::last_os_error())
	} else {
		Ok(ret)
	}
}

pub(crate) fn job_object(kill_on_drop: bool) -> Result<(HANDLE, HANDLE)> {
	let job = res_null(unsafe { CreateJobObjectW(ptr::null_mut(), ptr::null()) })?;

	let completion_port =
		res_null(unsafe { CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 1) })?;

	let mut associate_completion = JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
		CompletionKey: job,
		CompletionPort: completion_port,
	};

	res_bool(unsafe {
		SetInformationJobObject(
			job,
			JobObjectAssociateCompletionPortInformation,
			&mut associate_completion as *mut _ as LPVOID,
			mem::size_of_val(&associate_completion)
				.try_into()
				.expect("cannot safely cast to DWORD"),
		)
	})?;

	let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();

	if kill_on_drop {
		info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
	}

	res_bool(unsafe {
		SetInformationJobObject(
			job,
			JobObjectExtendedLimitInformation,
			&mut info as *mut _ as LPVOID,
			mem::size_of_val(&info)
				.try_into()
				.expect("cannot safely cast to DWORD"),
		)
	})?;

	Ok((job, completion_port))
}

// This is pretty terrible, but it's either this or we re-implement all of Rust's std::process just
// to get at PROCESS_INFORMATION!
fn resume_threads(child_process: HANDLE) -> Result<()> {
	let child_id = unsafe { GetProcessId(child_process) };

	let h = res_null(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) })?;
	let mut entry = THREADENTRY32 {
		dwSize: 28,
		cntUsage: 0,
		th32ThreadID: 0,
		th32OwnerProcessID: 0,
		tpBasePri: 0,
		tpDeltaPri: 0,
		dwFlags: 0,
	};

	let mut res = res_bool(unsafe { Thread32First(h, &mut entry) });
	while res.is_ok() {
		if entry.th32OwnerProcessID == child_id {
			let thread_handle = res_null(unsafe { OpenThread(0x0002, 0, entry.th32ThreadID) })?;
			res_neg(unsafe { ResumeThread(thread_handle) })?;
			res_bool(unsafe { CloseHandle(thread_handle) })?;
		}

		res = res_bool(unsafe { Thread32Next(h, &mut entry) });
	}

	res_bool(unsafe { CloseHandle(h) })
}

pub(crate) fn assign_child(handle: RawHandle, job: HANDLE) -> Result<()> {
	let handle = handle as _;
	res_bool(unsafe { AssignProcessToJobObject(job, handle) })?;
	resume_threads(handle)?;
	Ok(())
}
