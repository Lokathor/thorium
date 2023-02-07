extern crate alloc;

mod macros;

pub mod errhandlingapi;
pub mod hidpi;
pub mod hidsdi;
pub mod libloaderapi;
pub mod win_types;
pub mod winbase;
pub mod winuser;

#[inline]
fn string_from_utf16(utf16: &[u16]) -> String {
  core::char::decode_utf16(utf16.iter().copied())
    .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
    .collect()
}
