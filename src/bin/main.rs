use thorium::{
  win_types::{ZWString, HWND, LPARAM, LRESULT, UINT, WPARAM},
  winuser::{DefWindowProcW, WindowClass},
};

fn main() {
  let win_class = WindowClass {
    class_name: Some(ZWString::from("WinClassName")),
    wnd_proc: Some(win_proc),
    ..Default::default()
  };
  let atom = win_class.register().unwrap();
  println!("{atom:?}");

  atom.unregister().unwrap();
  println!("done.");
}

unsafe extern "system" fn win_proc(
  hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM,
) -> LRESULT {
  match msg {
    _ => DefWindowProcW(hwnd, msg, w_param, l_param),
  }
}
