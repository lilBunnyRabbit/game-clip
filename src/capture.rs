use crate::clip;
use crate::config;
use crate::utils;

use device_query::{DeviceQuery, DeviceState, Keycode};
use scrap;
use std::{
  io::ErrorKind::WouldBlock,
  thread,
  time::{Duration, Instant},
};
use utils::{circular_buffer::CircularBuffer, logger, notification};

enum Actions {
  SaveGif,
  _SaveRaw,
  None,
}

pub fn capture_frames(mut capturer: scrap::Capturer, dimensions: (usize, usize)) {
  logger::info("Capturing frames");
  let config = config::get();

  let mut frames: CircularBuffer<clip::ClipFrame> =
    CircularBuffer::new(config.duration * config.fps as usize);

  let device_state = DeviceState::new();
  let mut prev_keys: Vec<device_query::Keycode> = vec![];

  let frame_time = Duration::new(1, 0) / config.fps;

  let mut timer = Instant::now();

  loop {
    match capturer.frame() {
      Ok(frame) => {
        let delay = timer.elapsed().as_secs_f64();
        frames.add(clip::ClipFrame {
          frame: frame.to_vec(),
          delay: delay,
        });

        println!("FPS | {}", (1.0 / delay).round());

        timer = Instant::now();
        thread::sleep(frame_time);
      }
      Err(ref e) if e.kind() == WouldBlock => {}
      Err(_) => break,
    }

    let keys = device_state.get_keys();
    match match_keys(&keys, &prev_keys) {
      Actions::SaveGif => {
        print!("{:?}", keys);
        let cloned_frames = frames.clone_buffer(); // Heavy
        thread::spawn(move || {
          notification::send_notification("Saving clip");
          clip::save_gif(cloned_frames, dimensions);
        });
      }
      Actions::_SaveRaw => {}
      Actions::None => {}
    }
    prev_keys = keys;
  }
}

fn match_keys(
  keys: &Vec<device_query::Keycode>,
  prev_keys: &Vec<device_query::Keycode>,
) -> Actions {
  if keys != prev_keys {
    return Actions::None;
  }

  if keys.len() == 3 {
    if keys.contains(&Keycode::Numpad7)
      && keys.contains(&Keycode::Numpad8)
      && keys.contains(&Keycode::Numpad9)
    {
      return Actions::SaveGif;
    }

    if keys.contains(&Keycode::Key7)
      && keys.contains(&Keycode::Key8)
      && keys.contains(&Keycode::Key9)
    {
      return Actions::SaveGif;
    }
  }

  return Actions::None;
}
