#![allow(clippy::single_match)]

use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, RendererFlags},
  Sdl,
};
use fermium::{
  prelude::{SDL_SetHint, SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS},
  stdinc::SDL_TRUE,
};

//
fn main() {
  let sdl = Sdl::init(InitFlags::GAMECONTROLLER);
  unsafe {
    assert_eq!(
      SDL_SetHint(
        SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS.as_ptr().cast(),
        "1\0".as_ptr().cast(),
      ),
      SDL_TRUE
    );
  }

  /*
  let win = sdl
    .create_renderer_window(CreateWinArgs::default(), RendererFlags::default());
  */

  'the_loop: loop {
    while let Some((event, _)) = sdl.poll_events() {
      println!("{event:?}");
      match event {
        Event::Quit => break 'the_loop,
        Event::ControllerAdded { index } => {
          println!("Opening Controller {index}");
          core::mem::forget(sdl.open_game_controller(index).unwrap());
        }
        _ => {}
      }
    }
  }
}
