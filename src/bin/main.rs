#![allow(unused_imports)]

use core::ptr::null_mut;

use thorium::{
  errhandlingapi::OsResult, hidpi::*, hidsdi::*, win_types::*, winuser::*,
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
  // probably replace this with a Mutex? Apparently a low-contention Mutex is
  // actually faster in general than using a thread local.
  static CAP_DATABASE: RefCell<HashMap<HANDLE, HidInfo>> = RefCell::new(HashMap::new());
}

struct HidInfo {
  preparsed_data: RawInputDevicePreparsedData,
  caps: HidpCaps,
  input_button_caps: Box<[HidpButtonCaps]>,
  input_value_caps: Box<[HidpValueCaps]>,
}
impl core::fmt::Debug for HidInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut x = f.debug_struct("HidInfo");
    x.field("preparsed_data", &"[...]");
    x.field("caps", &self.caps);
    x.field("input_button_caps", &self.input_button_caps);
    x.field("input_value_caps", &self.input_value_caps);
    x.finish()
  }
}
impl HidInfo {
  pub fn try_new(
    preparsed_data: RawInputDevicePreparsedData,
  ) -> HidpResult<Self> {
    let caps = hidp_get_caps(&preparsed_data)?;
    //
    let input_button_caps = {
      let num_input_buttons = usize::from(caps.number_input_button_caps);
      let mut buf: Vec<HidpButtonCaps> = Vec::with_capacity(num_input_buttons);
      Vec::from(hidp_get_button_caps(
        HidpReportType::INPUT,
        buf.spare_capacity_mut(),
        &preparsed_data,
      )?)
      .into_boxed_slice()
    };
    //
    let input_value_caps = {
      let num_input_values = usize::from(caps.number_input_value_caps);
      let mut buf: Vec<HidpValueCaps> = Vec::with_capacity(num_input_values);
      Vec::from(hidp_get_value_caps(
        HidpReportType::INPUT,
        buf.spare_capacity_mut(),
        &preparsed_data,
      )?)
      .into_boxed_slice()
    };
    //
    Ok(Self { preparsed_data, caps, input_button_caps, input_value_caps })
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
        println!("name: {:?}", get_raw_input_device_name(handle));
        println!("info: {:?}", get_raw_input_device_info(handle));
        let preparsed_data = match RawInputDevicePreparsedData::try_new(handle)
        {
          Ok(preparsed_data) => preparsed_data,
          Err(e) => {
            println!("Couldn't get preparsed data for {handle:?}: {e:?}");
            return DefWindowProcW(hwnd, msg, w_param, l_param);
          }
        };
        match HidInfo::try_new(preparsed_data) {
          Ok(hid_caps) => {
            println!("got all caps info:");
            println!("=caps: {:?}", hid_caps.caps);
            for (i, button_cap) in hid_caps.input_button_caps.iter().enumerate()
            {
              println!("=buttons[{i}]: {button_cap:?}");
            }
            for (i, value_cap) in hid_caps.input_value_caps.iter().enumerate() {
              println!("=values[{i}]: {value_cap:?}");
            }
            CAP_DATABASE.with(|ref_cell| {
              let db: &mut HashMap<_, _> = &mut *ref_cell.borrow_mut();
              db.insert(handle, hid_caps);
            });
          }
          Err(e) => println!("Err getting caps for new device: {e:?}"),
        }
      } else {
        println!("INPUT_DEVICE_CHANGE removed: {handle:?}");
        CAP_DATABASE.with(|ref_cell| {
          let db: &mut HashMap<_, _> = &mut *ref_cell.borrow_mut();
          db.remove(&handle);
        });
      }
      //panic!();
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
      let mut buffer: Vec<u8> = vec![0_u8; required_size];
      let data = match get_raw_input_data(hrawinput, &mut buffer) {
        Ok(data) => data,
        Err(e) => {
          println!("rawinput err: {e:?}");
          return DefWindowProcW(hwnd, msg, w_param, l_param);
        }
      };
      parse_raw_input(data);
    }
    _ => (),
  };
  DefWindowProcW(hwnd, msg, w_param, l_param)
}

#[allow(unused)]
fn parse_raw_input(data: &RawInputData) {
  let handle = data.handle();

  if data.hid_count().unwrap() > 1 {
    // TODO: right now the program doesn't handle multi-report data properly!
    return;
  }
  let report = data.hid_raw_data().unwrap();

  CAP_DATABASE.with(|ref_cell| {
    let db: &HashMap<_, _> = &*ref_cell.borrow();
    let Some(hid) = db.get(&handle) else {
      return;
    };

    // BUTTONS
    let len = hidp_max_button_list_length(
      HidpReportType::INPUT,
      HidUsagePage::BUTTONS,
      &hid.preparsed_data,
    );
    let mut buf: Vec<USAGE> = vec![0; len];
    let pressed_result = hidp_get_buttons(
      HidpReportType::INPUT,
      HidUsagePage::BUTTONS,
      0,
      &mut buf,
      &hid.preparsed_data,
      report,
    );
    //println!("Get Buttons: {pressed_result:?}");

    // AXISES
    let value_results: Vec<_> = hid
      .input_value_caps
      .iter()
      .map(|value_cap| {
        let usage = value_cap.u.not_range().usage;
        let usage_str = match usage {
          0x30 => "X",
          0x31 => "Y",
          0x32 => "Z",
          0x33 => "Rx",
          0x34 => "Ry",
          0x35 => "Rz",
          0x36 => "Slider",
          0x37 => "Dial",
          0x38 => "Wheel",
          0x39 => "Hat",
          _ => "?",
        };
        let value = hidp_get_usage_value(
          HidpReportType::INPUT,
          value_cap.usage_page,
          0,
          usage,
          &hid.preparsed_data,
          report,
        )
        .map(|value| {
          let value = value as i32;
          // this scaling function doesn't properly account for minimums that
          // aren't zero!
          if value >= value_cap.logical_min && value <= value_cap.logical_max {
            value as f32 / value_cap.logical_max as f32
          } else {
            -1.0
          }
        });
        (usage_str, value)
      })
      .collect();
    //println!("Get Values: {value_results:?}");
  });
}
