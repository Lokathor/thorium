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
  pub type HMODULE = HANDLE;
  pub type HINSTANCE = HANDLE;
  pub type HICON = HANDLE;
  pub type HCURSOR = HANDLE;
  pub type HBRUSH = HANDLE;
  pub type HWND = HANDLE;
  pub type LPCWSTR = *const WCHAR;
  pub type UINT = c_uint;
  pub type int = c_int;
  pub type ULONG_PTR = usize;
  pub type WORD = c_ushort;

  pub type WNDPROC_nn =
    unsafe extern "system" fn(HWND, u32, usize, isize) -> isize;
  pub type WNDPROC = Option<WNDPROC_nn>;

  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[repr(transparent)]
  pub struct HANDLE(pub isize);
  impl HANDLE {
    #[inline]
    #[must_use]
    pub const fn is_null(self) -> bool {
      self.0 == 0
    }
    #[inline]
    #[must_use]
    pub const fn null() -> Self {
      Self(0)
    }
  }
}

pub mod winbase {
  use core::ptr::NonNull;

  use super::{errhandlingapi::*, win_types::*};

  #[link(name = "Kernel32")]
  extern "system" {
    /// MSDN: [FormatMessageW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
    fn FormatMessageW(
      flags: DWORD, source: LPCVOID, message_id: DWORD, language_id: DWORD,
      buffer: LPWSTR, size: DWORD, arguments: *mut va_list,
    ) -> DWORD;

    /// MSDN: [LocalFree](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
    fn LocalFree(mem: HLOCAL) -> HLOCAL;
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
    pub const unsafe fn from_nn(nn: NonNull<T>) -> Self {
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
          w_chars_excluding_null.try_into().unwrap(),
        );
        let nn: NonNull<[u16]> = NonNull::new(p).unwrap();
        Ok(unsafe { LocalBox::from_nn(nn) })
      }
    }
  }
}

pub mod errhandlingapi {
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

  /// Sets the thread-local "last error" value.
  #[inline]
  pub fn set_last_error(err_code: ErrorCode) {
    unsafe { SetLastError(err_code.0) }
  }
}

pub mod libloaderapi {
  use super::{
    errhandlingapi::{get_last_error, ErrorCode},
    win_types::*,
  };

  #[link(name = "Kernel32")]
  extern "system" {
    /// MSDN: [GetModuleHandleW](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    fn GetModuleHandleW(module_name: LPCWSTR) -> HMODULE;
  }

  #[inline]
  pub fn get_process_instance() -> Result<HINSTANCE, ErrorCode> {
    let handle = unsafe { GetModuleHandleW(core::ptr::null()) };
    if handle.is_null() {
      Err(get_last_error())
    } else {
      Ok(handle)
    }
  }
}

pub mod winuser {
  use super::{
    errhandlingapi::{get_last_error, ErrorCode},
    win_types::*,
  };

  #[link(name = "User32")]
  extern "system" {
    /// MSDN: [LoadCursorW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    fn LoadCursorW(instance: HINSTANCE, cursor_name: LPCWSTR) -> HCURSOR;
  }

  #[derive(Clone, Copy)]
  #[repr(C)]
  struct WNDCLASSEXW {
    pub size: UINT,
    pub style: UINT,
    pub wnd_proc: WNDPROC,
    pub cls_extra: int,
    pub wnd_extra: int,
    pub instance: HINSTANCE,
    pub icon: HICON,
    pub cursor: HCURSOR,
    pub background: HBRUSH,
    pub menu_name: LPCWSTR,
    pub class_name: LPCWSTR,
    pub small_icon: HICON,
  }
  impl Default for WNDCLASSEXW {
    /// Correctly assigns `size` and leaves all other fields zeroed.
    #[inline]
    #[must_use]
    fn default() -> Self {
      Self {
        size: core::mem::size_of::<Self>().try_into().unwrap(),
        ..unsafe { core::mem::zeroed() }
      }
    }
  }

  /// * MSDN: [MAKEINTRESOURCEW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
  #[allow(dead_code)]
  const fn make_int_resource_w(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
  }

  /// The predefined cursor styles.
  ///
  /// * MSDN: [LoadCursorW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
  #[repr(u16)]
  pub enum IDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    /// Standard arrow
    Arrow = 32512,
    /// Crosshair
    Cross = 32515,
    /// Hand
    Hand = 32649,
    /// Arrow and question mark
    Help = 32651,
    /// I-beam
    IBeam = 32513,
    /// Slashed circle
    No = 32648,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    /// Vertical arrow
    UpArrow = 32516,
    /// Hourglass
    Wait = 32514,
  }

  /// Load one of the predefined cursors.
  ///
  /// MSDN: [LoadCursorW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
  #[inline]
  pub fn load_predefined_cursor(
    cursor: IDCursor,
  ) -> Result<HCURSOR, ErrorCode> {
    let instance = HINSTANCE::null();
    let cursor_name = make_int_resource_w(cursor as WORD);
    // Safety: The enum limits the allowed values to being from the approved
    // list on MSDN.
    let hcursor = unsafe { LoadCursorW(instance, cursor_name) };
    if hcursor.is_null() {
      Err(get_last_error())
    } else {
      Ok(hcursor)
    }
  }
}
