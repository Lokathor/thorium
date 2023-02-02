#![allow(non_camel_case_types)]

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
pub type ATOM = WORD;
pub type WPARAM = UINT_PTR;
pub type UINT_PTR = usize;
pub type LPARAM = LONG_PTR;
pub type LONG_PTR = isize;
pub type LRESULT = LONG_PTR;

pub type WNDPROC_nn = unsafe extern "system" fn(
  hwnd: HWND,
  msg: UINT,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT;
pub type WNDPROC = Option<WNDPROC_nn>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BOOL(pub int);
impl From<bool> for BOOL {
  #[inline]
  #[must_use]
  fn from(value: bool) -> Self {
    BOOL(value as int)
  }
}
impl From<BOOL> for bool {
  #[inline]
  #[must_use]
  fn from(value: BOOL) -> Self {
    value.0 != 0
  }
}

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

#[derive(Clone)]
#[repr(transparent)]
pub struct ZWString(Vec<u16>);
impl ZWString {
  #[inline]
  #[must_use]
  pub fn as_ptr(&self) -> *const u16 {
    self.0.as_ptr()
  }
  #[inline]
  #[must_use]
  pub fn as_mut_ptr(&mut self) -> *mut u16 {
    self.0.as_mut_ptr()
  }
  #[inline]
  #[must_use]
  pub fn chars<'a>(&'a self) -> impl Iterator<Item = char> + 'a {
    let live_slice: &[u16] = &self.0[..(self.0.len() - 1)];
    core::char::decode_utf16(live_slice.iter().copied())
      .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
  }
}
impl core::fmt::Debug for ZWString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"")?;
    core::fmt::Display::fmt(self, f)?;
    write!(f, "\"")?;
    Ok(())
  }
}
impl core::fmt::Display for ZWString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for ch in self.chars() {
      write!(f, "{ch}")?;
    }
    Ok(())
  }
}
impl From<&str> for ZWString {
  #[inline]
  #[must_use]
  fn from(value: &str) -> Self {
    Self(value.encode_utf16().chain(Some(0_u16)).collect())
  }
}
