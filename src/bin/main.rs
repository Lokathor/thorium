#![allow(unused_imports)]

use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval},
  Sdl,
};
use bytemuck::{offset_of, Pod, Zeroable};
use gles31::{
  glDrawArrays, glDrawElements, glEnable, glVertexAttribPointer, GL_FALSE,
  GL_FLOAT, GL_TRIANGLES, GL_UNSIGNED_INT,
};
use std::mem::size_of;
use thorium::*;

const VERTEX_SHADER_SRC: &str = "#version 310 es
layout (location = 0) in vec3 aPos;

void main() {
  gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const FRAGMENT_SHADER_SRC: &str = "#version 310 es
precision mediump float;

out vec4 FragColor;

void main() {
  FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
";

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
struct Vertex {
  pub pos: [f32; 3],
}
unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}
impl Vertex {
  /// Always configure vertex attributes after the buffer object has been
  /// initialized as least once.
  pub fn config_vertex_attributes() {
    set_vertex_attrib_array_enabled(0, true);
    unsafe {
      glVertexAttribPointer(
        0,
        3,
        GL_FLOAT,
        GL_FALSE,
        size_of::<Vertex>().try_into().unwrap(),
        offset_of!(Vertex, pos) as *mut _,
      );
    }
    for x in 1..get_max_vertex_attribute_count() {
      set_vertex_attrib_array_enabled(x, false);
    }
  }
}

const VERTICES: &[Vertex] = &[
  Vertex { pos: [0.5, 0.5, 0.0] },
  Vertex { pos: [0.5, -0.5, 0.0] },
  Vertex { pos: [-0.5, -0.5, 0.0] },
  Vertex { pos: [-0.5, 0.5, 0.0] },
];

const ELEMENTS: &[[u32; 3]] = &[[0, 1, 3], [1, 2, 3]];

fn main() {
  // Initializes SDL2
  let sdl = Sdl::init(InitFlags::VIDEO | InitFlags::TIMER);

  // configure the intended GL context
  sdl.set_gl_profile(GlProfile::ES).unwrap();
  sdl.set_gl_context_major_version(3).unwrap();
  sdl.set_gl_context_minor_version(1).unwrap();
  sdl.set_gl_multisample_buffers(1).unwrap();
  sdl.set_gl_multisample_count(4).unwrap();
  sdl.set_gl_framebuffer_srgb_capable(true).unwrap();
  #[cfg(debug_assertions)]
  sdl.set_gl_context_flags(GlContextFlags::DEBUG).unwrap();

  // Makes the window with a GL Context.
  let win = sdl
    .create_gl_window(CreateWinArgs {
      title: "Thorium",
      resizable: true,
      ..Default::default()
    })
    .unwrap();

  // Try to get Vsync, but the program will probably run without it.
  win.set_swap_interval(GlSwapInterval::Vsync).ok();

  // load all GL functions
  if let Err(err_list) =
    unsafe { gles31::load_gl_functions(&|name| win.get_proc_address(name)) }
  {
    let s = if err_list.len() != 1 { "s" } else { "" };
    println!("The following GL function{s} did not load: {err_list:?}");
  }

  if win.supports_extension("GL_KHR_debug") {
    unsafe {
      gles31::glDebugMessageCallbackKHR(
        Some(gl_debug_print_callback),
        core::ptr::null(),
      )
    };
  }
  println!("Max Vertex Attributes: {}", get_max_vertex_attribute_count());

  let (mut win_width, mut win_height) = win.get_window_size();
  let (mut mouse_x, mut mouse_y) = (win_width, win_height);

  let vao = VertexArrayObject::new();
  vao.bind();

  let vbo = BufferObject::new();
  ArrayBuffer.bind(&vbo);
  ArrayBuffer.realloc_from(VERTICES, DrawHint::StaticDraw);

  // You must configure the attributes **AFTER** having put data into the buffer
  // at least once.
  Vertex::config_vertex_attributes();

  let ebo = BufferObject::new();
  ElementArrayBuffer.bind(&ebo);
  ElementArrayBuffer.realloc_from(ELEMENTS, DrawHint::StaticDraw);

  let vertex_shader = Shader::new(ShaderType::Vertex);
  vertex_shader.set_source(VERTEX_SHADER_SRC);
  vertex_shader.compile().unwrap();

  let fragment_shader = Shader::new(ShaderType::Fragment);
  fragment_shader.set_source(FRAGMENT_SHADER_SRC);
  fragment_shader.compile().unwrap();

  let program = Program::new();
  program.attach_shader(&vertex_shader);
  program.attach_shader(&fragment_shader);
  program.link().unwrap();
  program.use_program();

  // program "main loop".
  'the_loop: loop {
    // Process all events from this frame.
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_loop,
        Event::MouseMotion { x_win, y_win, .. } => {
          mouse_x = x_win;
          mouse_y = y_win;
        }
        Event::WindowResized { width, height, .. } => {
          win_width = width;
          win_height = height;
          if win_width > 0 && win_height > 0 {
            set_viewport(win_width as u32, win_height as u32);
          }
        }
        _ => (),
      }
    }

    // draw
    let r = mouse_x as f32 / win_width as f32;
    let g = mouse_y as f32 / win_height as f32;
    set_clear_color(r, g, 0.0, 1.0);
    clear();

    unsafe {
      glDrawArrays(GL_TRIANGLES, 0, 3);
      //glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as _);
    }

    // flip
    win.swap_window();
  }

  // All the cleanup is handled by the various drop impls.
}
