#![warn(clippy::missing_inline_in_public_items)]

use core::ffi::c_void;
use std::ptr::NonNull;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BufferTarget {
  ArrayBuffer = GL_ARRAY_BUFFER,
  ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER,
}
impl BufferTarget {
  #[inline]
  pub fn bind(self, buf: &BufferObject) {
    unsafe { glBindBuffer(self as u32, buf.0) };
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
        self as u32,
        data_bytes.len().try_into().unwrap(),
        data_bytes.as_ptr().cast::<c_void>(),
        hint as u32,
      );
    }
  }

  /// Update's a buffer's memory, starting at `index` elements into the buffer,
  /// with the `new_data`.
  ///
  /// If the starting position and new data slice together would go out of
  /// bounds of the buffer's allocation then it will cause error and copy
  /// nothing at all.
  #[inline]
  pub fn update_from<T: Pod>(self, index: usize, new_data: &[T]) {
    let data_bytes: &[u8] = bytemuck::cast_slice(new_data);
    unsafe {
      glBufferSubData(
        self as u32,
        (index * core::mem::size_of::<T>()).try_into().unwrap(),
        data_bytes.len().try_into().unwrap(),
        data_bytes.as_ptr().cast(),
      )
    }
  }

  /// Maps part of a buffer into the CPU's address space.
  ///
  /// The pointer returned will be valid until [`unmap`](Self::unmap) is called.
  ///
  /// When this fails, `None` is returned.
  #[inline]
  pub fn map_buffer_range(
    self, offset_bytes: usize, len: usize,
  ) -> Option<NonNull<[u8]>> {
    let access = GL_MAP_READ_BIT | GL_MAP_WRITE_BIT;
    let p: *mut u8 = unsafe {
      glMapBufferRange(
        self as u32,
        offset_bytes.try_into().unwrap(),
        len.try_into().unwrap(),
        access,
      )
      .cast()
    };
    let p_slice: *mut [u8] = core::ptr::slice_from_raw_parts_mut(p, len);
    NonNull::new(p_slice)
  }

  /// Un-maps the buffer, invalidating the mapped pointer.
  ///
  /// * **Returns:** If the un-map operation was successful. When the data in
  ///   the mapped buffer has been corrupted since the mapping started then this
  ///   will return `false`.
  #[inline]
  pub fn unmap(self) -> bool {
    0 != unsafe { glUnmapBuffer(self as u32) }
  }

  /// Runs your closure on the mapped memory.
  ///
  /// This attempts to map the range specified, and on success it turns the
  /// pointer into a temporary slice and calls your closure on the slice.
  ///
  /// * **Returns:** The return value of the [`unmap`](Self::unmap) call. Or
  ///   `None` if the mapping didn't succeed in the first place.
  #[inline]
  pub fn map_closure<F: FnOnce(&mut [u8])>(
    self, offset_bytes: usize, len: usize, f: F,
  ) -> Option<bool> {
    match self.map_buffer_range(offset_bytes, len) {
      Some(mut nn) => {
        let slice: &mut [u8] = unsafe { nn.as_mut() };
        f(slice);
        Some(self.unmap())
      }
      None => None,
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

#[inline]
pub fn release_shader_compiler() {
  unsafe { glReleaseShaderCompiler() }
}
