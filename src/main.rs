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
  // let buffer: Vec<u8> = vec![3, 1, 2, 5, 1, 3, 1, 4, 1, 2, 5, 1, 5, 5, 1, 5, 5, 1, 4];
  // compression::lz77(buffer);
  let display = match clip::get_display(0) {
    Ok(display) => display,
    Err(error) => panic!("Failed to get the display: {}", error),
  };

  let capturer = scrap::Capturer::new(display).unwrap();
  let dimensions = (capturer.width(), capturer.height());

  capture_frames(capturer, dimensions);
}

fn capture_frames(mut capturer: scrap::Capturer, dimensions: (usize, usize)) {
  logger::info("Capturing frames");

  let mut frames: Vec<clip::ClipFrame> = Vec::new();

  let device_state = DeviceState::new();
  let mut prev_keys = vec![];

  let config = config::get();

  let max_frames = config.duration * config.fps as usize;
  let frame_time = Duration::new(1, 0) / config.fps;

  let mut timer = Instant::now();

  loop {
    match capturer.frame() {
      Ok(frame) => {
        if frames.len() == max_frames {
          frames.remove(0);
        } else {
          logger::info(format!("Captured frame {}", frames.len()));
        }

        frames.push(clip::ClipFrame {
          frame: frame.to_vec(),
          delay: timer.elapsed().as_secs_f64(),
        });

        timer = Instant::now();
      }
      Err(ref e) if e.kind() == WouldBlock => {
        thread::sleep(frame_time);
      }
      Err(_) => break,
    }

    let keys = device_state.get_keys();
    if keys != prev_keys {
      if keys.len() == 3
        && (keys.contains(&Keycode::Numpad7)
          && keys.contains(&Keycode::Numpad8)
          && keys.contains(&Keycode::Numpad9))
        || (keys.contains(&Keycode::Key7)
          && keys.contains(&Keycode::Key8)
          && keys.contains(&Keycode::Key9))
      {
        print!("{:?}", keys);
        let cloned_frames = frames.clone();
        thread::spawn(move || {
          utils::send_notification("Saving clip");
          clip::save_gif(cloned_frames, dimensions);
        });
      }
    }
    prev_keys = keys;
  }
}
