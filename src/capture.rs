use crate::clip;
use crate::config;
use crate::frame;
use crate::utils;

use device_query::{DeviceQuery, DeviceState, Keycode};
use scrap;
use std::{
  io::ErrorKind::WouldBlock,
  thread,
  time::{Duration, Instant},
};
use utils::{circular_buffer::CircularBuffer, fps_display::FpsDisplay, logger};

enum Actions {
  SaveGif,
  SaveRaw,
  None,
}

pub fn capture_frames(mut capturer: scrap::Capturer, dimensions: (usize, usize)) {
  logger::info("Capturing frames");
  let config = config::get();

  let mut frames: CircularBuffer<frame::ClipFrame> =
    CircularBuffer::new(config.duration * config.fps as usize);

  let device_state = DeviceState::new();
  let mut prev_keys: Vec<device_query::Keycode> = vec![];

  let frame_time = Duration::new(1, 0) / config.fps;
  let mut fps_display = FpsDisplay::new(config.fps as usize);
  let mut timer = Instant::now();

  loop {
    match capturer.frame() {
      Ok(frame) => {
        let delay = timer.elapsed().as_secs_f64();
        frames.add(frame::ClipFrame {
          frame: frame.to_vec(),
          delay: delay,
        });

        fps_display.add(1.0 / delay);

        timer = Instant::now();
      }
      Err(ref e) if e.kind() == WouldBlock => {
        thread::sleep(frame_time);
      }
      Err(_) => break,
    }

    let keys = device_state.get_keys();
    match match_keys(&keys, &prev_keys) {
      Actions::SaveGif => {
        let buffer = frames.clone_buffer(); // Heavy
        std::thread::spawn(move || clip::save_gif(buffer, dimensions));
      }
      Actions::SaveRaw => {
        let buffer = frames.clone_buffer(); // Heavy
        std::thread::spawn(move || clip::save_raw(buffer, dimensions));
      }
      Actions::None => {}
    }
    prev_keys = keys;
  }
}

fn match_keys(
  keys: &Vec<device_query::Keycode>,
  prev_keys: &Vec<device_query::Keycode>,
) -> Actions {
  if keys == prev_keys {
    return Actions::None;
  }

  if keys.len() == 3 {
    // 7 + 8 + 9 => SaveGif
    if keys.contains(&Keycode::Key7)
      && keys.contains(&Keycode::Key8)
      && keys.contains(&Keycode::Key9)
    {
      return Actions::SaveGif;
    }

    // Num7 + Num8 + Num9 => SaveRaw
    if keys.contains(&Keycode::Numpad7)
      && keys.contains(&Keycode::Numpad8)
      && keys.contains(&Keycode::Numpad9)
    {
      return Actions::SaveRaw;
    }
  }

  return Actions::None;
}
