#![allow(unused_imports)]

use core::ptr::null_mut;

use thorium::{
  errhandlingapi::OsResult,
  hidpi::{
    HIDP_BUTTON_CAPS, HIDP_CAPS, HIDP_REPORT_TYPE, HIDP_VALUE_CAPS, USAGE,
  },
  win_types::*,
  winuser::*,
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

  destroy_window(hwnd).unwrap();
  atom.unregister().unwrap();
}

use std::{cell::RefCell, collections::HashMap};
std::thread_local! {
  /// Simple buffer for raw input processing.
  /// Should only be used during WM_INPUT as a place to copy the data to.
  static RAW_INPUT_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());

  static CAP_DATABASE: RefCell<HashMap<HANDLE, HidCapabilities>> = RefCell::new(HashMap::new());
}

struct HidCapabilities {
  preparsed_data: RawInputDevicePreparsedData,
  caps: HIDP_CAPS,
  button_caps: Vec<HIDP_BUTTON_CAPS>,
  value_caps: Vec<HIDP_VALUE_CAPS>,
}
impl core::fmt::Debug for HidCapabilities {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("Capabilities");
    x.field("preparsed_data", &"[...]");
    x.field("caps", &self.caps);
    x.field("button_caps", &self.button_caps);
    x.field("value_caps", &self.value_caps);
    x.finish()
  }
}
impl HidCapabilities {
  pub fn try_new(handle: HANDLE) -> OsResult<Self> {
    let preparsed_data = RawInputDevicePreparsedData::try_new(handle)?;
    //
    let caps = preparsed_data.get_caps()?;
    //
    let mut button_caps =
      Vec::with_capacity(usize::from(caps.number_input_button_caps));
    let button_caps_written = preparsed_data.get_button_caps(
      HIDP_REPORT_TYPE::INPUT,
      button_caps.spare_capacity_mut(),
    )?;
    unsafe { button_caps.set_len(usize::from(button_caps_written)) };
    //
    let mut value_caps =
      Vec::with_capacity(usize::from(caps.number_input_value_caps));
    let value_caps_written = preparsed_data.get_value_caps(
      HIDP_REPORT_TYPE::INPUT,
      value_caps.spare_capacity_mut(),
    )?;
    unsafe { value_caps.set_len(usize::from(value_caps_written)) };
    //
    Ok(Self { preparsed_data, caps, button_caps, value_caps })
  }
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
    WinMessage::INPUT_DEVICE_CHANGE => {
      let added = match w_param {
        1 => true,
        2 => false,
        other => {
          println!("illegal INPUT_DEVICE_CHANGE w_param: {other}");
          false
        }
      };
      let handle = HANDLE(l_param);
      if added {
        println!("INPUT_DEVICE_CHANGE added: {handle:?}");
        CAP_DATABASE.with(|ref_cell| {
          let db: &mut HashMap<_, _> = &mut *ref_cell.borrow_mut();
          match HidCapabilities::try_new(handle) {
            Ok(hid_caps) => {
              println!("got all caps info:");
              println!("=caps: {:?}", hid_caps.caps);
              for (i, button_cap) in hid_caps.button_caps.iter().enumerate() {
                println!("=buttons[{i}]: {button_cap:?}");
              }
              for (i, value_cap) in hid_caps.value_caps.iter().enumerate() {
                println!("=values[{i}]: {value_cap:?}");
              }
              db.insert(handle, hid_caps);
            }
            Err(e) => println!("Err getting caps for new device: {e:?}"),
          }
        });
      } else {
        println!("INPUT_DEVICE_CHANGE removed: {handle:?}");
        CAP_DATABASE.with(|ref_cell| {
          let db: &mut HashMap<_, _> = &mut *ref_cell.borrow_mut();
          db.remove(&handle);
        });
      }
      return 0;
    }
    WinMessage::INPUT => {
      let hrawinput = HANDLE(l_param);
      let required_size =
        match get_raw_input_data_required_buffer_size(hrawinput) {
          Ok(required_size) => required_size,
          Err(e) => {
            println!("rawinput err: {e:?}");
            return DefWindowProcW(hwnd, msg, w_param, l_param);
          }
        };
      RAW_INPUT_BUFFER.with(|ref_cell| {
        let buffer: &mut Vec<u8> = &mut *ref_cell.borrow_mut();
        buffer.resize(required_size, 0);
        let data = match get_raw_input_data(hrawinput, buffer) {
          Ok(data) => data,
          Err(e) => {
            println!("rawinput err: {e:?}");
            return;
          }
        };
        parse_raw_input(data);
      });
    }
    _ => (),
  };
  DefWindowProcW(hwnd, msg, w_param, l_param)
}

fn parse_raw_input(data: &RawInputData) {
  let handle = data.handle();
  // TODO: handle multiple reports per input data packet.

  CAP_DATABASE.with(|ref_cell| {
    let db: &mut HashMap<_, _> = &mut *ref_cell.borrow_mut();
    let Some(hid_capabilities) = db.get(&handle) else {
      return;
    };

    // BUTTONS
    let len = hid_capabilities
      .preparsed_data
      .get_max_usage_list_length(HIDP_REPORT_TYPE::INPUT);
    let mut button_usage_buf: Vec<USAGE> = vec![0; len];
    match hid_capabilities.preparsed_data.get_usages(
      HIDP_REPORT_TYPE::INPUT,
      hid_capabilities.button_caps[0].usage_page,
      &mut button_usage_buf,
      data.hid_raw_data().unwrap(),
    ) {
      Ok(buttons_on) => {
        button_usage_buf.truncate(buttons_on.try_into().unwrap());
      }
      Err(e) => println!("err: {e:?}"),
    }
    println!("Buttons: {button_usage_buf:?}");

    // AXISES
    print!("Axises: ");
    for value_cap in hid_capabilities.value_caps.iter() {
      let usage = value_cap.u.not_range().usage;
      let usage_state = hid_capabilities.preparsed_data.get_usage_value(
        HIDP_REPORT_TYPE::INPUT,
        value_cap.usage_page,
        usage,
        data.hid_raw_data().unwrap(),
      );
      //#[cfg(FALSE)]
      match usage {
        0x30 => print!("X= {usage_state:?}, "),
        0x31 => print!("Y= {usage_state:?}, "),
        0x32 => print!("Z= {usage_state:?}, "),
        0x35 => print!("Rz= {usage_state:?}, "),
        0x39 => print!("Hat= {usage_state:?}, "),
        _ => print!("0x{usage:02X}= {usage_state:?}, "),
      }
    }
    println!();
  });
}
