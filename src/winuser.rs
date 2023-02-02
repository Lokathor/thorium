use core::{ffi::c_int, mem::size_of, ptr::null};

use crate::macros::impl_bit_ops;

use super::{
  errhandlingapi::{get_last_error_here, LocatedErrorCode, OsResult},
  libloaderapi::get_process_instance,
  win_types::*,
};

#[link(name = "User32")]
extern "system" {
  /// MSDN: [LoadCursorW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
  fn LoadCursorW(instance: HINSTANCE, cursor_name: LPCWSTR) -> HCURSOR;

  /// MSDN: [RegisterClassExW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
  fn RegisterClassExW(wnd_class_ex_w: *const WNDCLASSEXW) -> ATOM;

  /// MSDN: [UnregisterClassW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
  fn UnregisterClassW(class_name: LPCWSTR, instance: HINSTANCE) -> BOOL;

  /// MSDN: [DefWindowProcW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
  pub fn DefWindowProcW(
    hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM,
  ) -> LRESULT;

  /// MSDN: [CreateWindowExW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
  fn CreateWindowExW(
    ex_style: DWORD, class_name: LPCWSTR, window_name: LPCWSTR, style: DWORD,
    x: int, y: int, width: int, height: int, wnd_parent: HWND, menu: HMENU,
    instance: HINSTANCE, create_param: LPVOID,
  ) -> HWND;

  /// MSDN: [DestroyWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
  fn DestroyWindow(hwnd: HWND) -> BOOL;

  /// MSDN: [RegisterRawInputDevices](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerrawinputdevices)
  fn RegisterRawInputDevices(
    raw_input_devices: *const RAWINPUTDEVICE, num_devices: UINT, size: UINT,
  ) -> BOOL;
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct WindowClassStyle(UINT);
impl_bit_ops!(WindowClassStyle);
impl WindowClassStyle {
  pub const BYTEALIGNCLIENT: Self = Self(0x1000);
  pub const BYTEALIGNWINDOW: Self = Self(0x2000);
  pub const CLASSDC: Self = Self(0x0040);
  pub const DBLCLKS: Self = Self(0x0008);
  pub const DROPSHADOW: Self = Self(0x00020000);
  pub const GLOBALCLASS: Self = Self(0x4000);
  pub const HREDRAW: Self = Self(0x0002);
  pub const NOCLOSE: Self = Self(0x0200);
  pub const OWNDC: Self = Self(0x0020);
  pub const PARENTDC: Self = Self(0x0080);
  pub const SAVEBITS: Self = Self(0x0800);
  pub const VREDRAW: Self = Self(0x0001);
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct WindowStyle(UINT);
impl_bit_ops!(WindowStyle);
impl WindowStyle {
  pub const BORDER: Self = Self(0x00800000);
  pub const CAPTION: Self = Self(0x00C00000);
  pub const CHILD: Self = Self(0x40000000);
  pub const CHILDWINDOW: Self = Self(0x40000000);
  pub const CLIPCHILDREN: Self = Self(0x02000000);
  pub const CLIPSIBLINGS: Self = Self(0x04000000);
  pub const DISABLED: Self = Self(0x08000000);
  pub const DLGFRAME: Self = Self(0x00400000);
  pub const GROUP: Self = Self(0x00020000);
  pub const HSCROLL: Self = Self(0x00100000);
  pub const ICONIC: Self = Self(0x20000000);
  pub const MAXIMIZE: Self = Self(0x01000000);
  pub const MAXIMIZEBOX: Self = Self(0x00010000);
  pub const MINIMIZE: Self = Self(0x20000000);
  pub const MINIMIZEBOX: Self = Self(0x00020000);
  pub const OVERLAPPED: Self = Self(0x00000000);
  pub const OVERLAPPEDWINDOW: Self = Self(
    Self::OVERLAPPED.0
      | Self::CAPTION.0
      | Self::SYSMENU.0
      | Self::THICKFRAME.0
      | Self::MINIMIZEBOX.0
      | Self::MAXIMIZEBOX.0,
  );
  pub const POPUP: Self = Self(0x80000000);
  pub const POPUPWINDOW: Self =
    Self(Self::POPUP.0 | Self::BORDER.0 | Self::SYSMENU.0);
  pub const SIZEBOX: Self = Self(0x00040000);
  pub const SYSMENU: Self = Self(0x00080000);
  pub const TABSTOP: Self = Self(0x00010000);
  pub const THICKFRAME: Self = Self(0x00040000);
  pub const TILED: Self = Self(0x00000000);
  pub const TILEDWINDOW: Self = Self(
    Self::OVERLAPPED.0
      | Self::CAPTION.0
      | Self::SYSMENU.0
      | Self::THICKFRAME.0
      | Self::MINIMIZEBOX.0
      | Self::MAXIMIZEBOX.0,
  );
  pub const VISIBLE: Self = Self(0x10000000);
  pub const VSCROLL: Self = Self(0x00200000);
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct WindowStyleExtended(UINT);
impl_bit_ops!(WindowStyleExtended);
impl WindowStyleExtended {
  pub const ACCEPTFILES: Self = Self(0x00000010);
  pub const APPWINDOW: Self = Self(0x00040000);
  pub const CLIENTEDGE: Self = Self(0x00000200);
  pub const COMPOSITED: Self = Self(0x02000000);
  pub const CONTEXTHELP: Self = Self(0x00000400);
  pub const CONTROLPARENT: Self = Self(0x00010000);
  pub const DLGMODALFRAME: Self = Self(0x00000001);
  pub const LAYERED: Self = Self(0x0008000);
  pub const LAYOUTRTL: Self = Self(0x00400000);
  pub const LEFT: Self = Self(0x00000000);
  pub const LEFTSCROLLBAR: Self = Self(0x00004000);
  pub const LTRREADING: Self = Self(0x00000000);
  pub const MDICHILD: Self = Self(0x00000040);
  pub const NOACTIVATE: Self = Self(0x08000000);
  pub const NOINHERITLAYOUT: Self = Self(0x00100000);
  pub const NOPARENTNOTIFY: Self = Self(0x00000004);
  pub const NOREDIRECTIONBITMAP: Self = Self(0x00200000);
  pub const OVERLAPPEDWINDOW: Self =
    Self(Self::WINDOWEDGE.0 | Self::CLIENTEDGE.0);
  pub const PALETTEWINDOW: Self =
    Self(Self::WINDOWEDGE.0 | Self::TOOLWINDOW.0 | Self::TOPMOST.0);
  pub const WS_EX_RIGHT: Self = Self(0x00001000);
  pub const RIGHTSCROLLBAR: Self = Self(0x00000000);
  pub const RTLREADING: Self = Self(0x00002000);
  pub const STATICEDGE: Self = Self(0x00020000);
  pub const TOOLWINDOW: Self = Self(0x00000080);
  pub const TOPMOST: Self = Self(0x00000008);
  pub const TRANSPARENT: Self = Self(0x00000020);
  pub const WINDOWEDGE: Self = Self(0x00000100);
}

#[derive(Clone, Copy)]
#[repr(C)]
struct WNDCLASSEXW {
  size: UINT,
  style: UINT,
  wnd_proc: WNDPROC,
  cls_extra: int,
  wnd_extra: int,
  instance: HINSTANCE,
  icon: HICON,
  cursor: HCURSOR,
  background: HBRUSH,
  menu_name: LPCWSTR,
  class_name: LPCWSTR,
  small_icon: HICON,
}

#[derive(Default)]
pub struct WindowClass {
  pub style: Option<WindowClassStyle>,
  pub wnd_proc: WNDPROC, // already an Option type
  pub cls_extra: Option<int>,
  pub wnd_extra: Option<int>,
  pub icon: Option<HICON>,
  pub cursor: Option<HCURSOR>,
  pub background: Option<HBRUSH>,
  pub menu_name: Option<ZWString>,
  pub class_name: Option<ZWString>,
  pub small_icon: Option<HICON>,
}
impl WindowClass {
  /// Registers the window class, giving an "atom" that identifies it.
  ///
  /// ## Failure
  /// * This will fail if you don't provide a `class_name`.
  /// * Other failures may also occur.
  #[inline]
  #[track_caller]
  pub fn register(&self) -> OsResult<WindowClassAtom> {
    const COLOR_WINDOW: c_int = 5;
    //
    let win_class_ex_w = WNDCLASSEXW {
      size: size_of::<WNDCLASSEXW>().try_into().unwrap(),
      style: self.style.unwrap_or(WindowClassStyle(0)).0,
      wnd_proc: self.wnd_proc.or(Some(DefWindowProcW)),
      cls_extra: self.cls_extra.unwrap_or(0),
      wnd_extra: self.wnd_extra.unwrap_or(0),
      instance: get_process_instance()?,
      icon: self.icon.unwrap_or(HANDLE::null()),
      cursor: self.cursor.unwrap_or_else(|| {
        load_predefined_cursor(IDCursor::Arrow).ok().unwrap_or(HANDLE::null())
      }),
      background: self.background.unwrap_or(HANDLE((1 + COLOR_WINDOW) as _)),
      menu_name: self
        .menu_name
        .as_ref()
        .map(|zws| zws.as_ptr())
        .unwrap_or(null()),
      class_name: self
        .class_name
        .as_ref()
        .map(|zws| zws.as_ptr())
        .unwrap_or(null()),
      small_icon: self.small_icon.unwrap_or(HANDLE::null()),
    };
    let atom = unsafe { RegisterClassExW(&win_class_ex_w) };
    if atom != 0 {
      Ok(WindowClassAtom(atom))
    } else {
      Err(get_last_error_here())
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct WindowClassAtom(ATOM);
impl WindowClassAtom {
  #[inline]
  #[track_caller]
  pub fn unregister(&self) -> OsResult<()> {
    let instance = get_process_instance()?;
    let class_name = self.0 as LPCWSTR;
    if unsafe { UnregisterClassW(class_name, instance) }.into() {
      Ok(())
    } else {
      Err(get_last_error_here())
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
#[track_caller]
pub fn load_predefined_cursor(
  cursor: IDCursor,
) -> Result<HCURSOR, LocatedErrorCode> {
  let instance = HINSTANCE::null();
  let cursor_name = make_int_resource_w(cursor as WORD);
  // Safety: The enum limits the allowed values to being from the approved
  // list on MSDN.
  let hcursor = unsafe { LoadCursorW(instance, cursor_name) };
  if hcursor.is_null() {
    Err(get_last_error_here())
  } else {
    Ok(hcursor)
  }
}

#[inline]
#[track_caller]
pub unsafe fn create_window(
  name: ZWString, class_atom: WindowClassAtom, style: WindowStyle,
  ex_style: WindowStyleExtended, x: i32, y: i32, width: i32, height: i32,
  wnd_parent: HWND, menu: HMENU, create_param: LPVOID,
) -> OsResult<HWND> {
  let class_name = class_atom.0 as LPCWSTR;
  let window_name = name.as_ptr();
  let instance = get_process_instance()?;
  let hwnd = unsafe {
    CreateWindowExW(
      ex_style.0,
      class_name,
      window_name,
      style.0,
      x,
      y,
      width,
      height,
      wnd_parent,
      menu,
      instance,
      create_param,
    )
  };
  if hwnd.is_null() {
    Err(get_last_error_here())
  } else {
    Ok(hwnd)
  }
}

#[inline]
#[track_caller]
pub fn destroy_window(hwnd: HWND) -> OsResult<()> {
  if unsafe { DestroyWindow(hwnd) }.into() {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WinMessage(pub UINT);
impl WinMessage {
  pub const CREATE: Self = Self(0x0001);
  pub const INPUT: Self = Self(0x00FF);
}

/// MSDN: [RAWINPUTDEVICE](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-rawinputdevice)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RAWINPUTDEVICE {
  pub usage_page: USHORT,
  pub usage: USHORT,
  pub flags: DWORD,
  pub target: HWND,
}

#[inline]
#[track_caller]
pub fn register_raw_input_devices(devices: &[RAWINPUTDEVICE]) -> OsResult<()> {
  let raw_input_devices: *const RAWINPUTDEVICE = devices.as_ptr();
  let num_devices: UINT = devices.len().try_into().unwrap();
  let size: UINT = size_of::<RAWINPUTDEVICE>().try_into().unwrap();
  if unsafe { RegisterRawInputDevices(raw_input_devices, num_devices, size) }
    .into()
  {
    Ok(())
  } else {
    Err(get_last_error_here())
  }
}
