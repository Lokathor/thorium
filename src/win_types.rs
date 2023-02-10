#![allow(non_camel_case_types)]

use core::{
  alloc::Layout,
  ffi::*,
  mem::align_of,
  ops::{Deref, DerefMut},
  ptr::NonNull,
};

pub type DWORD = c_ulong;
pub type LPCVOID = *const c_void;
pub type LPVOID = *mut c_void;
pub type PVOID = *mut c_void;
pub type LPWSTR = *mut WCHAR;
pub type WCHAR = wchar_t;
pub type wchar_t = u16;
pub type va_list = c_void;
pub type HLOCAL = HANDLE;
pub type HMODULE = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMENU = HANDLE;
pub type HDC = HANDLE;
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
pub type USHORT = c_ushort;
pub type ULONG = c_ulong;
pub type LONG = c_long;
pub type HRAWINPUT = HANDLE;
pub type BYTE = u8;
pub type UCHAR = c_uchar;
pub type CHAR = c_char;
pub type COLORREF = DWORD;

// should probably be a newtype?
pub type NTSTATUS = LONG;

pub type WNDPROC_nn = unsafe extern "system" fn(
  hwnd: HWND,
  msg: UINT,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT;
pub type WNDPROC = Option<WNDPROC_nn>;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
impl core::fmt::Debug for BOOL {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let b: bool = (*self).into();
    core::fmt::Debug::fmt(&b, f)
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BOOLEAN(pub BYTE);
impl From<bool> for BOOLEAN {
  #[inline]
  #[must_use]
  fn from(value: bool) -> Self {
    BOOLEAN(value as BYTE)
  }
}
impl From<BOOLEAN> for bool {
  #[inline]
  #[must_use]
  fn from(value: BOOLEAN) -> Self {
    value.0 != 0
  }
}
impl core::fmt::Debug for BOOLEAN {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let b: bool = (*self).into();
    core::fmt::Debug::fmt(&b, f)
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
  #[allow(clippy::needless_lifetimes)]
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

/// GlobalAlloc buffer of bytes that's aligned to `usize`.
#[repr(transparent)]
pub struct GlobalBuffer(NonNull<[u8]>);
unsafe impl Send for GlobalBuffer {}
unsafe impl Sync for GlobalBuffer {}
impl GlobalBuffer {
  pub fn new(byte_count: usize) -> Option<Self> {
    let layout =
      Layout::from_size_align(byte_count, align_of::<usize>()).unwrap();
    let p: *mut u8 = unsafe { alloc::alloc::alloc_zeroed(layout) };
    let slice_p: *mut [u8] = core::ptr::slice_from_raw_parts_mut(p, byte_count);
    NonNull::new(slice_p).map(GlobalBuffer)
  }
}
impl Deref for GlobalBuffer {
  type Target = [u8];
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0.as_ptr() }
  }
}
impl DerefMut for GlobalBuffer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0.as_ptr() }
  }
}
impl Drop for GlobalBuffer {
  fn drop(&mut self) {
    let slice_p = self.0.as_ptr();
    let layout =
      Layout::from_size_align(unsafe { (*slice_p).len() }, align_of::<usize>())
        .unwrap();
    unsafe { alloc::alloc::dealloc(slice_p as *mut u8, layout) }
  }
}
impl Clone for GlobalBuffer {
  fn clone(&self) -> Self {
    let mut the_clone = Self::new(self.len()).unwrap();
    the_clone.copy_from_slice(self);
    the_clone
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RECT {
  pub left: LONG,
  pub top: LONG,
  pub right: LONG,
  pub bottom: LONG,
}
