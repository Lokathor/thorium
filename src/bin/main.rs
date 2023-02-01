use thorium::errhandlingapi::ErrorCode;

fn main() {
  let error_message = ErrorCode(1).format_system_error().unwrap();
  let mut s: String = core::char::decode_utf16(error_message.iter().cloned())
    .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
    .collect();
  while s.ends_with(&['\r', '\n']) {
    s.pop();
  }
  println!("`{s}`");
}
