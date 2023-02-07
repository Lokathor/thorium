use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::{null, null_mut},
};

use crate::{
  errhandlingapi::ErrorCode, macros::impl_bit_ops, string_from_utf16,
};

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

  /// MSDN: [ShowWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
  fn ShowWindow(hwnd: HWND, cmd: int) -> BOOL;

  /// MSDN: [GetMessageW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
  fn GetMessageW(
    msg: *mut MSG, hwnd: HWND, msg_filter_min: UINT, msg_filter_max: UINT,
  ) -> BOOL;

  /// MSDN: [PostQuitMessage](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
  fn PostQuitMessage(exit_code: int);

  /// MSDN: [TranslateMessage](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
  fn TranslateMessage(msg: *const MSG) -> BOOL;

  /// MSDN: [DispatchMessageW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
  fn DispatchMessageW(msg: *const MSG) -> LRESULT;

  /// MSDN: [GetRawInputData](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdata)
  fn GetRawInputData(
    raw_input: HRAWINPUT, command: UINT, data: LPVOID, size: *mut UINT,
    header_size: UINT,
  ) -> UINT;

  /// MSDN: [GetRawInputDeviceInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdeviceinfow)
  fn GetRawInputDeviceInfoW(
    device: HANDLE, command: UINT, data: LPVOID, size: *mut UINT,
  ) -> UINT;
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
  pub const CLOSE: Self = Self(0x0010);
  pub const QUIT: Self = Self(0x0012);
  pub const INPUT_DEVICE_CHANGE: Self = Self(0x00FE);
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

#[inline]
pub fn show_window(hwnd: HWND, visible: bool) {
  unsafe { ShowWindow(hwnd, visible as _) };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct POINT {
  pub x: LONG,
  pub y: LONG,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MSG {
  // Keep these private! TranslateMessage and DispatchMessage are marked safe
  // because users can't make up their own fake messages.
  hwnd: HWND,
  message: UINT,
  w_param: WPARAM,
  l_param: LPARAM,
  time: DWORD,
  pt: POINT,
  private: DWORD,
}
impl MSG {
  #[inline]
  #[must_use]
  pub fn is_quit_message(&self) -> bool {
    WinMessage(self.message) == WinMessage::QUIT
  }

  #[inline]
  fn blank() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

/// Gets any message for this thread, regardless of if it targets a specific
/// window or not.
///
/// This **blocks** until a message is returned.
#[inline]
#[track_caller]
pub fn get_any_message() -> OsResult<MSG> {
  let mut msg = MSG::blank();
  let ret = unsafe { GetMessageW(&mut msg, HWND::null(), 0, 0) };
  if ret.0 == -1 {
    Err(get_last_error_here())
  } else {
    Ok(msg)
  }
}

#[inline]
pub fn post_quit_message(exit_code: int) {
  unsafe { PostQuitMessage(exit_code) }
}

#[inline]
pub fn translate_message(msg: &MSG) -> BOOL {
  unsafe { TranslateMessage(msg) }
}

#[inline]
pub fn dispatch_message(msg: &MSG) -> LRESULT {
  unsafe { DispatchMessageW(msg) }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawInputType(DWORD);
impl RawInputType {
  pub const MOUSE: Self = Self(0);
  pub const KEYBOARD: Self = Self(1);
  pub const HID: Self = Self(2);
}
impl core::fmt::Debug for RawInputType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      Self::MOUSE => write!(f, "RawInputType::MOUSE"),
      Self::KEYBOARD => write!(f, "RawInputType::KEYBOARD"),
      Self::HID => write!(f, "RawInputType::HID"),
      Self(other) => write!(f, "RawInputType({other}"),
    }
  }
}

/// MSDN: [RAWINPUTHEADER](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-rawinputheader)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct RAWINPUTHEADER {
  ty: RawInputType,
  size: DWORD,
  device: HANDLE,
  w_param: WPARAM,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RAWMOUSE_DummyStructName {
  button_flags: USHORT,
  button_data: USHORT,
}

#[derive(Clone, Copy)]
#[repr(C)]
union RAWMOUSE_DummyUnionName {
  buttons: ULONG,
  dummy: RAWMOUSE_DummyStructName,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RAWMOUSE {
  flags: USHORT,
  dummy: RAWMOUSE_DummyUnionName,
  raw_buttons: ULONG,
  last_x: LONG,
  last_y: LONG,
  extra_information: ULONG,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct RAWKEYBOARD {
  make_code: USHORT,
  flags: USHORT,
  reserved: USHORT,
  v_key: USHORT,
  message: UINT,
  extra_information: ULONG,
}

/// MSDN: [RAWHID](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-rawhid)
#[derive(Clone, Copy)]
#[repr(C)]
struct RAWHID {
  size_hid: DWORD,
  count: DWORD,
  raw_data: [BYTE; 1], // DANGER!! Flexable Array Member!
}

#[derive(Clone, Copy)]
pub struct RawHidView<'a>(&'a [u8]);

#[derive(Clone, Copy)]
#[repr(C)]
union RAWINPUT_union {
  mouse: RAWMOUSE,
  keyboard: RAWKEYBOARD,
  hid: RAWHID,
}

/// MSDN: [RAWINPUT](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-rawinput)
#[derive(Clone, Copy)]
#[repr(C)]
struct RAWINPUT {
  header: RAWINPUTHEADER,
  data: RAWINPUT_union,
}

/// Gets the required buffer size for [get_raw_input_data] to be successful.
#[inline]
#[track_caller]
pub fn get_raw_input_data_required_buffer_size(
  raw_input: HRAWINPUT,
) -> OsResult<usize> {
  const RID_INPUT: UINT = 0x10000003;
  const RAWINPUTHEADER_SIZE: UINT = size_of::<RAWINPUTHEADER>() as u32;
  //
  let mut size: UINT = 0;
  let ret = unsafe {
    GetRawInputData(
      raw_input,
      RID_INPUT,
      null_mut(),
      &mut size,
      RAWINPUTHEADER_SIZE,
    )
  };
  if ret != UINT::MAX {
    Ok(size.try_into().unwrap())
  } else {
    Err(get_last_error_here())
  }
}

/// Copes the RawInput data into the buffer, returning the byte count written.
///
/// To determine the required buffer size call
/// [get_raw_input_data_required_buffer_size].
#[inline]
#[track_caller]
pub fn get_raw_input_data(
  raw_input: HRAWINPUT, buf: &mut [u8],
) -> OsResult<&RawInputData> {
  const RID_INPUT: UINT = 0x10000003;
  const RAWINPUTHEADER_SIZE: UINT = size_of::<RAWINPUTHEADER>() as u32;
  //
  let mut size: UINT = buf.len().try_into().unwrap();
  let ret = unsafe {
    GetRawInputData(
      raw_input,
      RID_INPUT,
      buf.as_mut_ptr().cast::<c_void>(),
      &mut size,
      RAWINPUTHEADER_SIZE,
    )
  };
  if ret != UINT::MAX {
    let size_usize: usize = size.try_into().unwrap();
    let buf_sized: &[u8] = &buf[..size_usize];
    let raw_input_data: &RawInputData =
      unsafe { core::mem::transmute(buf_sized) };
    Ok(raw_input_data)
  } else {
    Err(get_last_error_here())
  }
}

#[repr(transparent)]
pub struct RawInputData([u8]);

impl RawInputData {
  /// Gets the raw input device handle associated with this data.
  #[inline]
  pub fn handle(&self) -> HANDLE {
    let buf: &[u8] = &self.0;
    if buf.len() < size_of::<RAWINPUTHEADER>() {
      // this shouldn't happen, but if it does i guess we shouldn't panic
      HANDLE::null()
    } else {
      unsafe { (*buf.as_ptr().cast::<RAWINPUT>()).header.device }
    }
  }

  #[inline]
  pub fn input_type(&self) -> RawInputType {
    let buf: &[u8] = &self.0;
    if buf.len() < size_of::<RAWINPUTHEADER>() {
      RawInputType(u32::MAX)
    } else {
      unsafe { (*buf.as_ptr().cast::<RAWINPUT>()).header.ty }
    }
  }

  /// `self.hid.size_hid`, fails if this isn't an HID
  #[inline]
  pub fn hid_size_hid(&self) -> Option<DWORD> {
    if self.input_type() != RawInputType::HID {
      return None;
    }
    let full_buffer: &[u8] = &self.0;
    let data_buffer = if full_buffer.len() >= size_of::<RAWINPUTHEADER>() {
      full_buffer.split_at(size_of::<RAWINPUTHEADER>()).1
    } else {
      return None;
    };
    Some(DWORD::from_ne_bytes(data_buffer[0..4].try_into().unwrap()))
  }

  /// `self.hid.count`, fails if this isn't an HID
  #[inline]
  pub fn hid_count(&self) -> Option<DWORD> {
    if self.input_type() != RawInputType::HID {
      return None;
    }
    let full_buffer: &[u8] = &self.0;
    let data_buffer = if full_buffer.len() >= size_of::<RAWINPUTHEADER>() {
      full_buffer.split_at(size_of::<RAWINPUTHEADER>()).1
    } else {
      return None;
    };
    Some(DWORD::from_ne_bytes(data_buffer[4..8].try_into().unwrap()))
  }

  /// `self.hid.raw_data`, fails if this isn't an HID
  #[inline]
  pub fn hid_raw_data(&self) -> Option<&[u8]> {
    if self.input_type() != RawInputType::HID {
      return None;
    }
    let full_buffer: &[u8] = &self.0;
    let data_buffer = if full_buffer.len() >= size_of::<RAWINPUTHEADER>() {
      full_buffer.split_at(size_of::<RAWINPUTHEADER>()).1
    } else {
      return None;
    };
    if data_buffer.len() >= (2 * size_of::<DWORD>()) {
      Some(&data_buffer[(2 * size_of::<DWORD>())..])
    } else {
      None
    }
  }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct RawInputDevicePreparsedData(pub(crate) GlobalBuffer);
impl RawInputDevicePreparsedData {
  #[inline]
  #[track_caller]
  pub fn try_new(device: HANDLE) -> OsResult<Self> {
    const RIDI_PREPARSEDDATA: UINT = 0x20000005;
    //
    let mut size: UINT = 0;
    let get_required_ret = unsafe {
      GetRawInputDeviceInfoW(device, RIDI_PREPARSEDDATA, null_mut(), &mut size)
    };
    if get_required_ret != 0 {
      return Err(get_last_error_here());
    }
    let mut buf = GlobalBuffer::new(size.try_into().unwrap())
      .ok_or(LocatedErrorCode::new(ErrorCode::NOT_ENOUGH_MEMORY))?;
    let bytes_written = unsafe {
      GetRawInputDeviceInfoW(
        device,
        RIDI_PREPARSEDDATA,
        buf.as_mut_ptr().cast(),
        &mut size,
      )
    };
    if bytes_written != buf.len().try_into().unwrap() {
      Err(get_last_error_here())
    } else {
      Ok(Self(buf))
    }
  }
}

#[inline]
#[track_caller]
pub fn get_raw_input_device_name(device: HANDLE) -> OsResult<String> {
  const RIDI_DEVICENAME: UINT = 0x20000007;
  //
  let mut char_count: UINT = 0;
  let get_required_ret = unsafe {
    GetRawInputDeviceInfoW(device, RIDI_DEVICENAME, null_mut(), &mut char_count)
  };
  if get_required_ret != 0 {
    return Err(get_last_error_here());
  }
  let mut buf: Vec<u16> = Vec::with_capacity(char_count.try_into().unwrap());
  let bytes_written = unsafe {
    GetRawInputDeviceInfoW(
      device,
      RIDI_DEVICENAME,
      buf.as_mut_ptr().cast(),
      &mut char_count,
    )
  };
  if bytes_written == u32::MAX {
    Err(get_last_error_here())
  } else {
    // count is -1 because the last unit is the 0 terminator
    unsafe { buf.set_len(char_count.saturating_sub(1).try_into().unwrap()) };
    Ok(string_from_utf16(&buf))
  }
}
