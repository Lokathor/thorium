use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ShaderType {
  Vertex = GL_VERTEX_SHADER,
  Fragment = GL_FRAGMENT_SHADER,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Shader(pub(crate) u32);
impl Drop for Shader {
  #[inline]
  fn drop(&mut self) {
    unsafe { glDeleteShader(self.0) }
  }
}
impl Shader {
  #[inline]
  pub fn new(ty: ShaderType) -> Self {
    Self(unsafe { glCreateShader(ty as u32) })
  }
  #[inline]
  pub fn set_source(&self, src: &str) {
    let strings: [*const u8; 1] = [src.as_ptr()];
    let lengths: [i32; 1] = [src.len().try_into().unwrap()];
    assert_eq!(strings.len(), lengths.len());
    unsafe {
      glShaderSource(
        self.0,
        strings.len().try_into().unwrap(),
        strings.as_ptr(),
        lengths.as_ptr(),
      )
    }
  }
  #[inline]
  pub fn get_shader_type(&self) -> Option<ShaderType> {
    let mut param: i32 = 0;
    unsafe { glGetShaderiv(self.0, GL_SHADER_TYPE, &mut param) };
    match param as u32 {
      GL_VERTEX_SHADER => Some(ShaderType::Vertex),
      GL_FRAGMENT_SHADER => Some(ShaderType::Fragment),
      _ => None,
    }
  }
  #[inline]
  pub fn get_last_compile_successful(&self) -> bool {
    let mut param: i32 = 0;
    unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut param) };
    param != 0
  }
  #[inline]
  pub fn get_info_log_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut param) };
    param.try_into().unwrap()
  }
  #[inline]
  pub fn get_shader_source_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetShaderiv(self.0, GL_SHADER_SOURCE_LENGTH, &mut param) };
    param.try_into().unwrap()
  }
  #[inline]
  pub fn compile(&self) -> Result<(), String> {
    unsafe { glCompileShader(self.0) };
    if self.get_last_compile_successful() {
      Ok(())
    } else {
      Err(self.get_info_log())
    }
  }
  #[inline]
  pub fn get_info_log(&self) -> String {
    let required_capacity = self.get_info_log_capacity_requirement();
    let mut vec: Vec<u8> = Vec::with_capacity(required_capacity);
    let capacity: u32 = vec.capacity().try_into().unwrap();
    let mut length: u32 = 0;
    unsafe {
      glGetShaderInfoLog(self.0, capacity, &mut length, vec.as_mut_ptr());
      vec.set_len(length.try_into().unwrap());
    }
    match String::from_utf8(vec) {
      Ok(string) => string,
      Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    }
  }
}
