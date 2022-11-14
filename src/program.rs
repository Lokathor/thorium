use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum GlDataTy {
  F32 = GL_FLOAT,
  Vec2 = GL_FLOAT_VEC2,
  Vec3 = GL_FLOAT_VEC3,
  Vec4 = GL_FLOAT_VEC4,
  I32 = GL_INT,
  IVec2 = GL_INT_VEC2,
  IVec3 = GL_INT_VEC3,
  IVec4 = GL_INT_VEC4,
  U32 = GL_UNSIGNED_INT,
  UVec2 = GL_UNSIGNED_INT_VEC2,
  UVec3 = GL_UNSIGNED_INT_VEC3,
  UVec4 = GL_UNSIGNED_INT_VEC4,
  Bool = GL_BOOL,
  BVec2 = GL_BOOL_VEC2,
  BVec3 = GL_BOOL_VEC3,
  BVec4 = GL_BOOL_VEC4,
  Mat2 = GL_FLOAT_MAT2,
  Mat3 = GL_FLOAT_MAT3,
  Mat4 = GL_FLOAT_MAT4,
  Mat2x3 = GL_FLOAT_MAT2x3,
  Mat2x4 = GL_FLOAT_MAT2x4,
  Mat3x2 = GL_FLOAT_MAT3x2,
  Mat3x4 = GL_FLOAT_MAT3x4,
  Mat4x2 = GL_FLOAT_MAT4x2,
  Mat4x3 = GL_FLOAT_MAT4x3,
  Sampler2d = GL_SAMPLER_2D,
  Sampler3d = GL_SAMPLER_3D,
  SamplerCube = GL_SAMPLER_CUBE,
  Sampler2dShadow = GL_SAMPLER_2D_SHADOW,
  Sampler2dArray = GL_SAMPLER_2D_ARRAY,
  Sampler2dArrayShadow = GL_SAMPLER_2D_ARRAY_SHADOW,
  SamplerCubeShadow = GL_SAMPLER_CUBE_SHADOW,
  ISampler2d = GL_INT_SAMPLER_2D,
  ISampler3d = GL_INT_SAMPLER_3D,
  ISamplerCube = GL_INT_SAMPLER_CUBE,
  ISampler2dArray = GL_INT_SAMPLER_2D_ARRAY,
  USampler2d = GL_UNSIGNED_INT_SAMPLER_2D,
  USampler3d = GL_UNSIGNED_INT_SAMPLER_3D,
  USamplerCube = GL_UNSIGNED_INT_SAMPLER_CUBE,
  USampler2dArray = GL_UNSIGNED_INT_SAMPLER_2D_ARRAY,
  Unknown = 0,
}
impl GlDataTy {
  #[inline]
  #[allow(non_upper_case_globals)]
  fn new(u: u32) -> Self {
    match u {
      GL_FLOAT => GlDataTy::F32,
      GL_FLOAT_VEC2 => GlDataTy::Vec2,
      GL_FLOAT_VEC3 => GlDataTy::Vec3,
      GL_FLOAT_VEC4 => GlDataTy::Vec4,
      GL_INT => GlDataTy::I32,
      GL_INT_VEC2 => GlDataTy::IVec2,
      GL_INT_VEC3 => GlDataTy::IVec3,
      GL_INT_VEC4 => GlDataTy::IVec4,
      GL_UNSIGNED_INT => GlDataTy::U32,
      GL_UNSIGNED_INT_VEC2 => GlDataTy::UVec2,
      GL_UNSIGNED_INT_VEC3 => GlDataTy::UVec3,
      GL_UNSIGNED_INT_VEC4 => GlDataTy::UVec4,
      GL_BOOL => GlDataTy::Bool,
      GL_BOOL_VEC2 => GlDataTy::BVec2,
      GL_BOOL_VEC3 => GlDataTy::BVec3,
      GL_BOOL_VEC4 => GlDataTy::BVec4,
      GL_FLOAT_MAT2 => GlDataTy::Mat2,
      GL_FLOAT_MAT3 => GlDataTy::Mat3,
      GL_FLOAT_MAT4 => GlDataTy::Mat4,
      GL_FLOAT_MAT2x3 => GlDataTy::Mat2x3,
      GL_FLOAT_MAT2x4 => GlDataTy::Mat2x4,
      GL_FLOAT_MAT3x2 => GlDataTy::Mat3x2,
      GL_FLOAT_MAT3x4 => GlDataTy::Mat3x4,
      GL_FLOAT_MAT4x2 => GlDataTy::Mat4x2,
      GL_FLOAT_MAT4x3 => GlDataTy::Mat4x3,
      GL_SAMPLER_2D => GlDataTy::Sampler2d,
      GL_SAMPLER_3D => GlDataTy::Sampler3d,
      GL_SAMPLER_CUBE => GlDataTy::SamplerCube,
      GL_SAMPLER_2D_SHADOW => GlDataTy::Sampler2dShadow,
      GL_SAMPLER_2D_ARRAY => GlDataTy::Sampler2dArray,
      GL_SAMPLER_2D_ARRAY_SHADOW => GlDataTy::Sampler2dArrayShadow,
      GL_SAMPLER_CUBE_SHADOW => GlDataTy::SamplerCubeShadow,
      GL_INT_SAMPLER_2D => GlDataTy::ISampler2d,
      GL_INT_SAMPLER_3D => GlDataTy::ISampler3d,
      GL_INT_SAMPLER_CUBE => GlDataTy::ISamplerCube,
      GL_INT_SAMPLER_2D_ARRAY => GlDataTy::ISampler2dArray,
      GL_UNSIGNED_INT_SAMPLER_2D => GlDataTy::USampler2d,
      GL_UNSIGNED_INT_SAMPLER_3D => GlDataTy::USampler3d,
      GL_UNSIGNED_INT_SAMPLER_CUBE => GlDataTy::USamplerCube,
      GL_UNSIGNED_INT_SAMPLER_2D_ARRAY => GlDataTy::USampler2dArray,
      _ => GlDataTy::Unknown,
    }
  }
}

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
  pub fn get_active_attribute_count(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_ACTIVE_ATTRIBUTES, &mut param) }
    param.try_into().unwrap()
  }
  /// The capacity required to fit any Attribute name.
  #[inline]
  pub fn get_active_attribute_name_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe {
      glGetProgramiv(self.0, GL_ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut param)
    }
    param.try_into().unwrap()
  }
  /// Gets the `(Name, ArraySize, Type)` of the attribute at `index`
  #[inline]
  pub fn get_active_attribute(
    &self, index: usize,
  ) -> (String, usize, GlDataTy) {
    let required_capacity =
      self.get_active_attribute_name_capacity_requirement();
    let mut vec: Vec<u8> = Vec::with_capacity(required_capacity);
    let capacity: u32 = vec.capacity().try_into().unwrap();
    let mut length: u32 = 0;
    let mut array_size: i32 = 0;
    let mut type_: u32 = 0;
    unsafe {
      glGetActiveAttrib(
        self.0,
        index.try_into().unwrap(),
        capacity,
        &mut length,
        &mut array_size,
        &mut type_,
        vec.as_mut_ptr(),
      );
      vec.set_len(length.try_into().unwrap());
    }
    let string = match String::from_utf8(vec) {
      Ok(string) => string,
      Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    };
    (string, array_size.try_into().unwrap(), GlDataTy::new(type_))
  }

  #[inline]
  pub fn get_active_uniform_block_count(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_ACTIVE_UNIFORM_BLOCKS, &mut param) }
    param.try_into().unwrap()
  }
  /// The capacity required to fit any Uniform Block name.
  #[inline]
  pub fn get_active_uniform_block_name_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe {
      glGetProgramiv(
        self.0,
        GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
        &mut param,
      )
    }
    param.try_into().unwrap()
  }

  #[inline]
  pub fn get_active_uniform_count(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_ACTIVE_UNIFORMS, &mut param) }
    param.try_into().unwrap()
  }
  /// The capacity required to fit any Uniform's name.
  #[inline]
  pub fn get_active_uniform_name_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_ACTIVE_UNIFORM_MAX_LENGTH, &mut param) }
    param.try_into().unwrap()
  }
  /// Gets the `(Name, ArraySize, Type, Location)` of the uniform at `index`
  #[inline]
  pub fn get_active_uniform(
    &self, index: usize,
  ) -> (String, usize, GlDataTy, i32) {
    let required_capacity = self.get_active_uniform_name_capacity_requirement();
    let mut vec: Vec<u8> = Vec::with_capacity(required_capacity);
    let capacity: u32 = vec.capacity().try_into().unwrap();
    let mut length: u32 = 0;
    let mut array_size: i32 = 0;
    let mut type_: u32 = 0;
    unsafe {
      glGetActiveUniform(
        self.0,
        index.try_into().unwrap(),
        capacity,
        &mut length,
        &mut array_size,
        &mut type_,
        vec.as_mut_ptr(),
      );
      vec.set_len(length.try_into().unwrap());
    }
    let location = unsafe { glGetUniformLocation(self.0, vec.as_ptr().cast()) };
    let string = match String::from_utf8(vec) {
      Ok(string) => string,
      Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    };
    (string, array_size.try_into().unwrap(), GlDataTy::new(type_), location)
  }
  // TODO: glGetActiveUniformsiv has more info we could query

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
  pub fn get_validate_status(&self) -> bool {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_VALIDATE_STATUS, &mut param) }
    param != 0
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
  pub fn validate(&self) -> Result<(), String> {
    unsafe { glValidateProgram(self.0) }
    if self.get_validate_status() {
      Ok(())
    } else {
      Err(self.get_info_log())
    }
  }
  #[inline]
  pub fn use_program(&self) {
    unsafe { glUseProgram(self.0) }
  }

  /// The number of bytes required to save the program's binary.
  #[inline]
  pub fn get_binary_capacity_requirement(&self) -> usize {
    let mut param: i32 = 0;
    unsafe { glGetProgramiv(self.0, GL_PROGRAM_BINARY_LENGTH, &mut param) }
    param.try_into().unwrap()
  }
  #[inline]
  pub fn get_binary(&self) -> (u32, Vec<u8>) {
    let required_capacity = self.get_binary_capacity_requirement();
    let mut vec: Vec<u8> = Vec::with_capacity(required_capacity);
    let capacity: u32 = vec.capacity().try_into().unwrap();
    let mut length: u32 = 0;
    let mut format: u32 = 0;
    unsafe {
      glGetProgramBinary(
        self.0,
        capacity,
        &mut length,
        &mut format,
        vec.as_mut_ptr().cast(),
      );
      vec.set_len(length.try_into().unwrap());
    }
    (format, vec)
  }
  #[inline]
  pub fn set_binary(&self, format: u32, data: Vec<u8>) {
    let length: u32 = data.len().try_into().unwrap();
    unsafe { glProgramBinary(self.0, format, data.as_ptr().cast(), length) }
  }
}
