use core::panic::Location;

use super::win_types::*;

/// The error code bit that indicates an application error.
///
/// If you are defining an error code for your application, set this bit to
/// indicate that the error code has been defined by your application and to
/// ensure that your error code does not conflict with any system-defined
/// error codes.
pub const APPLICATION_ERROR_BIT: DWORD = 1 << 29;

#[link(name = "Kernel32")]
extern "system" {
  /// MSDN: [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
  fn GetLastError() -> DWORD;

  /// MSDN: [SetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
  fn SetLastError(err_code: DWORD);
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ErrorCode(pub DWORD);
impl core::fmt::Debug for ErrorCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ErrorCode(0x{:08X})", self.0)
  }
}
impl core::fmt::Display for ErrorCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "0x{:08X}", self.0)
  }
}

/// Gets the thread-local "last error" value.
#[inline]
#[must_use]
pub fn get_last_error() -> ErrorCode {
  ErrorCode(unsafe { GetLastError() })
}

/// Sets the thread-local "last error" value.
#[inline]
pub fn set_last_error(err_code: ErrorCode) {
  unsafe { SetLastError(err_code.0) }
}

/// Bundles an error code and [Location] into a single struct.
#[derive(Clone, Copy)]
pub struct LocatedErrorCode {
  pub location: &'static Location<'static>,
  pub err_code: ErrorCode,
}
impl LocatedErrorCode {
  #[inline]
  #[must_use]
  #[track_caller]
  pub fn new(err_code: ErrorCode) -> Self {
    Self { err_code, location: Location::caller() }
  }
}
impl core::fmt::Debug for LocatedErrorCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let file = self.location.file();
    let line = self.location.line();
    let column = self.location.column();
    let err_code = self.err_code;
    let system_error = self.err_code.format_system_error();
    let u16_slice: &[u16] = system_error.as_deref().unwrap_or(&[]);
    let mut err_msg: String =
      core::char::decode_utf16(u16_slice.iter().copied())
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect();
    while err_msg.ends_with(&['\r', '\n']) {
      err_msg.pop();
    }
    write!(f, "[{file}:{line}:{column}]({err_code}): {err_msg}")
  }
}

/// Gets the last error wrapped with a [Location].
#[inline]
#[must_use]
#[track_caller]
pub fn get_last_error_here() -> LocatedErrorCode {
  LocatedErrorCode::new(get_last_error())
}

pub type OsResult<T> = Result<T, LocatedErrorCode>;
