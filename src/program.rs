use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Program(u32);
impl Drop for Program {
  #[inline]
  fn drop(&mut self) {
    unsafe { glDeleteProgram(self.0) }
  }
}
impl Program {
  #[inline]
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self(unsafe { glCreateProgram() })
  }
  #[inline]
  pub fn attach_shader(&self, shader: &Shader) {
    unsafe { glAttachShader(self.0, shader.0) }
  }
  #[inline]
  pub fn link(&self) -> Result<(), String> {
    unsafe { glLinkProgram(self.0) }
    if self.get_last_link_successful() {
      Ok(())
    } else {
      Err(self.get_info_log())
    }
  }
  #[inline]
  pub fn get_last_link_successful(&self) -> bool {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut param) }
    param != 0
  }
  #[inline]
  pub fn get_info_log_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut param) }
    param.try_into().unwrap()
  }
  #[inline]
  pub fn get_info_log(&self) -> String {
    let required_capacity = self.get_info_log_capacity_requirement();
    let mut vec: Vec<u8> = Vec::with_capacity(required_capacity);
    let capacity: u32 = vec.capacity().try_into().unwrap();
    let mut length: u32 = 0;
    unsafe {
      glGetProgramInfoLog(self.0, capacity, &mut length, vec.as_mut_ptr());
      vec.set_len(length.try_into().unwrap());
    }
    match String::from_utf8(vec) {
      Ok(string) => string,
      Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    }
  }
  #[inline]
  pub fn use_program(&self) {
    unsafe { glUseProgram(self.0) }
  }
  // TODO: other `glGetProgramiv` information
}
