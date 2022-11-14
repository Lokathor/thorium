#![warn(clippy::missing_inline_in_public_items)]

use core::ffi::c_void;

use bytemuck::Pod;
use gles31::*;

mod debug;
pub use debug::*;

mod shader;
pub use shader::*;

mod program;
pub use program::*;

#[inline]
pub fn set_viewport(win_width: u32, win_height: u32) {
  unsafe { glViewport(0, 0, win_width, win_height) }
}

#[inline]
pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
  unsafe { glClearColor(r, g, b, a) }
}

#[inline]
pub fn clear() {
  const MASK: u32 =
    GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT;
  unsafe { glClear(MASK) }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexArrayObject(u32);
impl VertexArrayObject {
  #[inline]
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let mut vao = 0;
    unsafe { glGenVertexArrays(1, &mut vao) };
    Self(vao)
  }
}
impl Drop for VertexArrayObject {
  #[inline]
  fn drop(&mut self) {
    if self.0 != 0 {
      unsafe { glDeleteVertexArrays(1, &self.0) };
    }
  }
}
impl VertexArrayObject {
  #[inline]
  pub fn bind(&self) {
    unsafe { glBindVertexArray(self.0) }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferObject(u32);
impl BufferObject {
  #[inline]
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let mut buf = 0;
    unsafe { glGenBuffers(1, &mut buf) };
    Self(buf)
  }
}
impl Drop for BufferObject {
  #[inline]
  fn drop(&mut self) {
    if self.0 != 0 {
      unsafe { glDeleteBuffers(1, &self.0) };
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum DrawHint {
  StaticDraw = GL_STATIC_DRAW,
}

pub struct ArrayBuffer;
impl ArrayBuffer {
  #[inline]
  pub fn bind(self, buf: &BufferObject) {
    unsafe { glBindBuffer(GL_ARRAY_BUFFER, buf.0) };
  }

  /// Reallocates the bound buffer to the required size and then copies the
  /// provided data into the buffer's new allocation.
  ///
  /// * `hint` is how you intend to use the data during drawing.
  #[inline]
  pub fn realloc_from<T: Pod>(self, data: &[T], hint: DrawHint) {
    let data_bytes: &[u8] = bytemuck::cast_slice(data);
    unsafe {
      glBufferData(
        GL_ARRAY_BUFFER,
        data_bytes.len().try_into().unwrap(),
        data_bytes.as_ptr().cast::<c_void>(),
        hint as u32,
      );
    }
  }
}

pub struct ElementArrayBuffer;
impl ElementArrayBuffer {
  #[inline]
  pub fn bind(self, buf: &BufferObject) {
    unsafe { glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, buf.0) };
  }

  /// Reallocates the bound buffer to the required size and then copies the
  /// provided data into the buffer's new allocation.
  ///
  /// * `hint` is how you intend to use the data during drawing.
  #[inline]
  pub fn realloc_from<T: Pod>(self, data: &[T], hint: DrawHint) {
    let data_bytes: &[u8] = bytemuck::cast_slice(data);
    unsafe {
      glBufferData(
        GL_ELEMENT_ARRAY_BUFFER,
        data_bytes.len().try_into().unwrap(),
        data_bytes.as_ptr().cast::<c_void>(),
        hint as u32,
      );
    }
  }
}

#[inline]
pub fn get_max_vertex_attribute_count() -> u32 {
  let mut data: i32 = 0;
  unsafe { glGetIntegerv(GL_MAX_VERTEX_ATTRIBS, &mut data) }
  data as u32
}

#[inline]
pub fn set_vertex_attrib_array_enabled(attrib_index: u32, enabled: bool) {
  if enabled {
    unsafe { glEnableVertexAttribArray(attrib_index) };
  } else {
    unsafe { glDisableVertexAttribArray(attrib_index) };
  }
}
