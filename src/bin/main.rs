#![allow(unused_imports)]

use core::ptr::null_mut;

use thorium::{hidpi::HIDP_REPORT_TYPE, win_types::*, winuser::*};

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
    WinMessage::CLOSE => {
      post_quit_message(0);
      return 0;
    }
    WinMessage::INPUT => {
      let hrawinput = HANDLE(l_param);
      let data = match RawInputData::try_new(hrawinput) {
        Ok(data) => data,
        Err(e) => {
          println!("Err getting raw input data: {e:?}");
          return DefWindowProcW(hwnd, msg, w_param, l_param);
        }
      };
      parse_raw_input(data);
    }
    _ => (),
  };
  DefWindowProcW(hwnd, msg, w_param, l_param)
}

fn parse_raw_input(data: RawInputData) {
  let handle = data.handle();
  let preparsed_data = RawInputDevicePreparsedData::try_new(handle).unwrap();
  let caps = preparsed_data.get_caps().unwrap();
  println!("== {caps:?}");
  let mut button_caps =
    Vec::with_capacity(usize::from(caps.number_input_button_caps));
  let button_caps_written = preparsed_data
    .get_button_caps(HIDP_REPORT_TYPE::INPUT, button_caps.spare_capacity_mut())
    .unwrap();
  unsafe { button_caps.set_len(usize::from(button_caps_written)) };
  println!("== {button_caps:?}");
  println!("== Number Of Buttons: {}", unsafe {
    button_caps[0].u.range.usage_max - button_caps[0].u.range.usage_min + 1
  });
}
