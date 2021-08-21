mod circular_buffer;
mod clip;
mod compression;
mod config;
mod logger;
mod utils;

use device_query::{DeviceQuery, DeviceState, Keycode};
use scrap;
use std::{
  io::ErrorKind::WouldBlock,
  thread,
  time::{Duration, Instant},
};

fn main() {
  let display = match clip::get_display(0) {
    Ok(display) => display,
    Err(error) => panic!("Failed to get the display: {}", error),
  };

  let capturer = scrap::Capturer::new(display).unwrap();
  let dimensions = (capturer.width(), capturer.height());

  capture_frames(capturer, dimensions);
}

enum Actions {
  SaveGif,
  _SaveRaw,
  None,
}

fn capture_frames(mut capturer: scrap::Capturer, dimensions: (usize, usize)) {
  logger::info("Capturing frames");
  let config = config::get();

  let mut frames: circular_buffer::CircularBuffer<clip::ClipFrame> =
    circular_buffer::CircularBuffer::new(config.duration * config.fps as usize);

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
        let cloned_frames = frames.clone_buffer();
        thread::spawn(move || {
          utils::send_notification("Saving clip");
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
