use thorium::errhandlingapi::ErrorCode;

fn main() {
  let buf = ErrorCode(1).format_to_buffer().unwrap();
  let mut s: String = core::char::decode_utf16(buf.iter().cloned())
    .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
    .collect();
  while s.ends_with(&['\r', '\n']) {
    s.pop();
  }
  println!("`{s}`");
}
