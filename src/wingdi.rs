use super::{
  errhandlingapi::{get_last_error_here, OsResult},
  win_types::*,
};

#[link(name = "User32")]
extern "system" {
  /// MSDN: [SetBkMode](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
  fn SetBkMode(hdc: HDC, mode: int) -> int;
}

const TRANSPARENT: int = 1;
const OPAQUE: int = 2;

#[repr(i32)]
pub enum BackgroundMode {
  Transparent = TRANSPARENT,
  Opaque = OPAQUE,
}

/// Sets the background mix mode, giving the previous mode.
#[inline]
#[track_caller]
pub fn set_background_mode(
  hdc: HDC, mode: BackgroundMode,
) -> OsResult<BackgroundMode> {
  let out = unsafe { SetBkMode(hdc, mode as int) };
  match out {
    TRANSPARENT => Ok(BackgroundMode::Transparent),
    OPAQUE => Ok(BackgroundMode::Opaque),
    _ => Err(get_last_error_here()),
  }
}

#[inline]
#[allow(non_snake_case)]
pub const fn RGB(r: BYTE, g: BYTE, b: BYTE) -> COLORREF {
  r as COLORREF | ((g as COLORREF) << 8) | ((b as COLORREF) << 16)
}
