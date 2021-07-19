use std::{
	convert::TryInto,
	io::Result,
	mem,
	os::windows::{io::AsRawHandle, process::CommandExt},
	process::Command,
	ptr,
};
use winapi::{
	shared::minwindef::LPVOID,
	um::{
		handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
		ioapiset::CreateIoCompletionPort,
		jobapi2::{AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject},
		processthreadsapi::{GetProcessId, OpenThread, ResumeThread},
		tlhelp32::{
			CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
		},
		winbase::CREATE_SUSPENDED,
		winnt::{
			JobObjectAssociateCompletionPortInformation, JobObjectExtendedLimitInformation, HANDLE,
			JOBOBJECT_ASSOCIATE_COMPLETION_PORT, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
			JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
		},
	},
};

use crate::{winres::*, CommandGroup, GroupChild};

impl CommandGroup for Command {
	fn group_spawn(&mut self) -> Result<GroupChild> {
		let job = res_null(unsafe { CreateJobObjectW(ptr::null_mut(), ptr::null()) })?;

		let completion_port = res_null(unsafe {
			CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 1)
		})?;

		let mut associate_completion = JOBOBJECT_ASSOCIATE_COMPLETION_PORT::default();
		associate_completion.CompletionKey = job;
		associate_completion.CompletionPort = completion_port;

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
		info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
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

		self.creation_flags(CREATE_SUSPENDED);

		let child = self.spawn()?;

		let handle = child.as_raw_handle() as _;
		res_bool(unsafe { AssignProcessToJobObject(job, handle) })?;
		resume_threads(handle)?;

		Ok(GroupChild::new(child, job, completion_port))
	}
}

// This is pretty terrible, but it's either this or we re-implement all of Rust's std::process just
// to get at PROCESS_INFORMATION!
fn resume_threads(child_process: HANDLE) -> Result<()> {
	unsafe {
		let child_id = GetProcessId(child_process);

		let h = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
		let mut entry = THREADENTRY32 {
			dwSize: 28,
			cntUsage: 0,
			th32ThreadID: 0,
			th32OwnerProcessID: 0,
			tpBasePri: 0,
			tpDeltaPri: 0,
			dwFlags: 0,
		};

		let mut result = Thread32First(h, &mut entry);
		while result != 0 {
			if entry.th32OwnerProcessID == child_id {
				let thread_handle = OpenThread(0x0002, 0, entry.th32ThreadID);
				ResumeThread(thread_handle);
				CloseHandle(thread_handle);
			}

			result = Thread32Next(h, &mut entry);
		}

		CloseHandle(h);
	}

	Ok(())
}
