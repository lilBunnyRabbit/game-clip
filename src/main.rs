mod clip;
mod logger;

use device_query::{DeviceQuery, DeviceState, Keycode};
use notify_rust::Notification;
use scrap;
use std::{io::ErrorKind::WouldBlock, thread};

fn main() {
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

  let mut frames: Vec<Vec<u8>> = Vec::new();

  let device_state = DeviceState::new();
  let mut prev_keys = vec![];

  let max_frames = clip::SETTINGS.duration * clip::SETTINGS.fps;
  let frame_time = clip::get_frame_time();

  loop {
    match capturer.frame() {
      Ok(frame) => {
        if frames.len() == max_frames {
          frames.remove(0);
        } else {
          logger::info(format!("Captured frame {}", frames.len()));
        }

        frames.push(frame.to_vec());
      }
      Err(ref e) if e.kind() == WouldBlock => {
        thread::sleep(frame_time);
      }
      Err(_) => break,
    }

    let keys = device_state.get_keys();
    if keys != prev_keys {
      if keys.len() == 3
        && keys.contains(&Keycode::LAlt)
        && keys.contains(&Keycode::G)
        && keys.contains(&Keycode::C)
      {
        print!("{:?}", keys);
        let cloned_frames = frames.clone();
        thread::spawn(move || {
          send_notification("Clipping screen");
          clip::save_gif(cloned_frames, dimensions);
        });
      }
    }
    prev_keys = keys;
  }
}

fn send_notification(message: &str) {
  match Notification::new().summary(message).show() {
    Ok(_) => {}
    Err(_) => {}
  };
}
