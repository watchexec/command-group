use std::io::{Error, Result};
use winapi::{shared::minwindef::{BOOL, DWORD}, um::winnt::HANDLE};

pub(crate) fn res_null(handle: HANDLE) -> Result<HANDLE> {
	if handle.is_null() {
		Err(Error::last_os_error())
	} else {
		Ok(handle)
	}
}

pub(crate) fn res_bool(ret: BOOL) -> Result<()> {
	if ret == 0 {
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
