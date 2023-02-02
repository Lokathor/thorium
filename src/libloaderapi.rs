use super::{
  errhandlingapi::{get_last_error_here, LocatedErrorCode},
  win_types::*,
};

#[link(name = "Kernel32")]
extern "system" {
  /// MSDN: [GetModuleHandleW](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  fn GetModuleHandleW(module_name: LPCWSTR) -> HMODULE;
}

#[inline]
#[track_caller]
pub fn get_process_instance() -> Result<HINSTANCE, LocatedErrorCode> {
  let handle = unsafe { GetModuleHandleW(core::ptr::null()) };
  if handle.is_null() {
    Err(get_last_error_here())
  } else {
    Ok(handle)
  }
}
