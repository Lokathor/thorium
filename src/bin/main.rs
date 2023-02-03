#![allow(unused_imports)]

use core::ptr::null_mut;

use thorium::{
  win_types::{ZWString, HMENU, HWND, LPARAM, LRESULT, UINT, WPARAM},
  winuser::{
    create_window, dispatch_message, get_any_message, post_quit_message,
    register_raw_input_devices, show_window, translate_message, DefWindowProcW,
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

  show_window(hwnd, true);

  loop {
    match get_any_message() {
      Ok(msg) => {
        if msg.is_quit_message() {
          break;
        } else {
          translate_message(&msg);
          dispatch_message(&msg);
        }
      }
      Err(e) => eprintln!("GetMessage Error: {e:?}"),
    }
  }

  // Note: When DefWindowProcW handles a CLOSE message it will also destroy the
  // window, so for now we don't need to manual destroy it.

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
    WinMessage::CLOSE => {
      post_quit_message(0);
      DefWindowProcW(hwnd, msg, w_param, l_param)
    }
    _ => DefWindowProcW(hwnd, msg, w_param, l_param),
  }
}
