use core::ptr::null_mut;

use thorium::{
  win_types::{ZWString, HMENU, HWND, LPARAM, LRESULT, UINT, WPARAM},
  winuser::{
    create_window, destroy_window, register_raw_input_devices, DefWindowProcW,
    WinMessage, WindowClass, WindowStyle, WindowStyleExtended, RAWINPUTDEVICE,
  },
};

fn main() {
  let win_class = WindowClass {
    class_name: Some(ZWString::from("WinClassName")),
    wnd_proc: Some(win_proc),
    ..Default::default()
  };
  let atom = win_class.register().unwrap();
  println!("Window Class Atom: {atom:?}");

  let style = WindowStyle::OVERLAPPEDWINDOW;
  let ex_style = WindowStyleExtended::APPWINDOW;
  let hwnd = unsafe {
    create_window(
      ZWString::from("The Window"),
      atom,
      style,
      ex_style,
      i32::MIN,
      i32::MIN,
      800,
      600,
      HWND::null(),
      HMENU::null(),
      null_mut(),
    )
  }
  .unwrap();
  println!("Window Handle: {hwnd:?}");

  // TODO: show window?

  // TODO: run event loop until close.

  destroy_window(hwnd).unwrap();
  atom.unregister().unwrap();
}

unsafe extern "system" fn win_proc(
  hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM,
) -> LRESULT {
  match WinMessage(msg) {
    WinMessage::CREATE => {
      const HID_PAGE_GENERIC_DESKTOP: u16 = 1;
      const HID_PAGE_GENERIC_DESKTOP__JOYSTICK: u16 = 4;
      //const HID_PAGE_GENERIC_DESKTOP__GAMEPAD: u16 = 5;
      //const HID_PAGE_GENERIC_DESKTOP__MULTIAXIS_CONTROLLER: u16 = 8;
      const RIDEV_INPUTSINK: u32 = 0x00000100;
      const RIDEV_DEVNOTIFY: u32 = 0x00002000;
      let devices = [RAWINPUTDEVICE {
        usage_page: HID_PAGE_GENERIC_DESKTOP,
        usage: HID_PAGE_GENERIC_DESKTOP__JOYSTICK,
        flags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY,
        target: hwnd,
      }];
      if let Err(e) = register_raw_input_devices(&devices) {
        println!("raw input register err: {e:?}");
        return -1;
      };
      //
      return 0;
    }
    _ => DefWindowProcW(hwnd, msg, w_param, l_param),
  }
}
