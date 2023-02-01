#[allow(non_camel_case_types)]
pub mod win_types {
  use core::ffi::*;

  pub type DWORD = c_ulong;
  pub type LPCVOID = *const c_void;
  pub type LPWSTR = *mut WCHAR;
  pub type WCHAR = wchar_t;
  pub type wchar_t = u16;
  pub type va_list = c_void;
  pub type HLOCAL = HANDLE;

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[repr(transparent)]
  pub struct HANDLE(pub isize);
}

pub mod winbase {
  use core::ptr::NonNull;

  use super::{errhandlingapi::*, win_types::*};

  #[link(name = "Kernel32")]
  extern "system" {
    /// MSDN: [FormatMessageW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
    pub fn FormatMessageW(
      flags: DWORD, source: LPCVOID, message_id: DWORD, language_id: DWORD,
      buffer: LPWSTR, size: DWORD, arguments: *mut va_list,
    ) -> DWORD;

    /// MSDN: [LocalFree](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
    pub fn LocalFree(mem: HLOCAL) -> HLOCAL;
  }

  /// Owned data allocated by `LocalAlloc`.
  ///
  /// Gets dropped by `LocalFree`.
  #[derive(Debug)]
  #[repr(transparent)]
  pub struct LocalBox<T: ?Sized>(NonNull<T>);
  impl<T: ?Sized> LocalBox<T> {
    /// ## Safety
    /// * The pointer must have been allocated with `LocalAlloc` or
    ///   `LocalReAlloc`.
    /// * The pointer must point to initialized data.
    /// * This passes ownership of the pointer into the function.
    #[inline]
    pub const unsafe fn new(nn: NonNull<T>) -> Self {
      Self(nn)
    }
  }
  impl<T: ?Sized> Drop for LocalBox<T> {
    #[inline]
    fn drop(&mut self) {
      let handle = HANDLE(self.0.as_ptr() as *mut u8 as isize);
      unsafe { LocalFree(handle) };
    }
  }
  impl<T: ?Sized> core::ops::Deref for LocalBox<T> {
    type Target = T;
    #[inline]
    #[must_use]
    fn deref(&self) -> &T {
      unsafe { &*self.0.as_ptr() }
    }
  }
  impl<T: ?Sized> core::ops::DerefMut for LocalBox<T> {
    #[inline]
    #[must_use]
    fn deref_mut(&mut self) -> &mut T {
      unsafe { &mut *self.0.as_ptr() }
    }
  }

  impl ErrorCode {
    /// Formats an error code from the system into UTF-16 text.
    ///
    /// ## Failure
    /// * Application errors can't be formatted with this method.
    pub fn format_system_error(self) -> Result<LocalBox<[u16]>, ErrorCode> {
      // https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew#parameters
      const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;
      const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
      const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

      let mut local_alloc_ptr: *mut u16 = core::ptr::null_mut();

      let flags = FORMAT_MESSAGE_ALLOCATE_BUFFER
        | FORMAT_MESSAGE_FROM_SYSTEM
        | FORMAT_MESSAGE_IGNORE_INSERTS;
      let source = 0 as _;
      let message_id = self.0;
      let language_id = 0;
      let buffer = &mut local_alloc_ptr as *mut *mut u16 as *mut u16;
      let size = 0;
      let arguments = 0 as _;

      let w_chars_excluding_null = unsafe {
        FormatMessageW(
          flags,
          source,
          message_id,
          language_id,
          buffer,
          size,
          arguments,
        )
      };
      if w_chars_excluding_null == 0 || local_alloc_ptr.is_null() {
        Err(get_last_error())
      } else {
        let p: *mut [u16] = core::ptr::slice_from_raw_parts_mut(
          local_alloc_ptr,
          w_chars_excluding_null as usize,
        );
        let nn: NonNull<[u16]> = NonNull::new(p).unwrap();
        Ok(unsafe { LocalBox::new(nn) })
      }
    }
  }
}

pub mod errhandlingapi {
  use super::win_types::*;

  /// If you are defining an error code for your application, set this bit to
  /// indicate that the error code has been defined by your application and to
  /// ensure that your error code does not conflict with any system-defined
  /// error codes.
  pub const APPLICATION_ERROR_BIT: DWORD = 1 << 29;

  #[link(name = "Kernel32")]
  extern "system" {
    /// MSDN: [GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;

    /// MSDN: [SetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
    pub fn SetLastError(err_code: DWORD);
  }

  #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[repr(transparent)]
  pub struct ErrorCode(pub DWORD);
  impl core::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "ErrorCode(0x{:08X})", self.0)
    }
  }
  impl core::fmt::Debug for ErrorCode {
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
}
