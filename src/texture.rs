use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Texture(u32);
impl Texture {
  #[inline]
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let mut vao = 0;
    unsafe { glGenTextures(1, &mut vao) };
    Self(vao)
  }
}
impl Drop for Texture {
  #[inline]
  fn drop(&mut self) {
    if self.0 != 0 {
      unsafe { glDeleteTextures(1, &self.0) };
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TextureTarget {
  Texture2d = GL_TEXTURE_2D,
  Texture3d = GL_TEXTURE_3D,
  Texture2dArray = GL_TEXTURE_2D_ARRAY,
  TextureCubeMap = GL_TEXTURE_CUBE_MAP,
}
impl TextureTarget {
  #[inline]
  pub fn bind(self, tex: &Texture) {
    unsafe { glBindTexture(self as u32, tex.0) };
  }
}

/// Sets the alignment of each pixel row to be 1, 2, 4, or 8.
///
/// The default is 4.
#[inline]
pub fn set_pixel_unpack_alignment(n: i32) {
  unsafe { glPixelStorei(GL_UNPACK_ALIGNMENT, n) }
}
