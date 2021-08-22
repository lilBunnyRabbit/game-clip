use crate::config;
use crate::frame;
use crate::utils::{logger, notification};

use gifski;
use std::{fs::File, thread, time::SystemTime};

pub struct WriterProgress {}
impl gifski::progress::ProgressReporter for WriterProgress {
  fn increase(&mut self) -> bool {
    logger::info("Progress increased");
    true
  }
  fn done(&mut self, msg: &str) {
    logger::info(format!("Progress done: {}", msg));
  }
}

fn init_gifski(_dimensions: (usize, usize)) -> (gifski::Collector, gifski::Writer) {
  let config = config::get();
  let gif_settings = gifski::Settings {
    width: Some(config.width),
    height: Some(config.height),
    quality: config.quality,
    fast: config.fast,
    repeat: config.repeat,
  };

  logger::info("Gifski init");
  return gifski::new(gif_settings).unwrap();
}

pub fn save_gif(frames: Vec<frame::ClipFrame>, dimensions: (usize, usize)) {
  let config = config::get();
  let (mut collector, writer) = init_gifski(dimensions);

  let mut timestamp: f64 = 0.0;

  let collector_thread = thread::spawn(move || {
    for (i, frame) in frames.iter().enumerate() {
      if i != 0 {
        timestamp += frame.delay;
      }
      let imgvec = frame::frame_to_imgvec(dimensions.0, dimensions.1, &frame.frame);

      match collector.add_frame_rgba(i, imgvec, timestamp) {
        Ok(_) => logger::info(format!("Added frame {} at {}, ", i, timestamp)),
        Err(error) => panic!("Err adding frame {}", error),
      }
    }
    drop(collector);
    logger::info("Dropped collector");
  });

  let writer_thread = thread::spawn(move || {
    let now: std::time::Duration = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .expect("Couldn't get Epoch time");
    let filename = format!("{}{}.gif", config.path, now.as_millis());

    let file = File::create(filename.as_str()).unwrap();
    logger::info(format!("Created file '{}'", filename));

    let progress_reporter: &mut dyn gifski::progress::ProgressReporter = &mut WriterProgress {};

    logger::info("Writing frames");
    match writer.write(file, progress_reporter) {
      Ok(_) => {
        logger::info(format!("Gif '{}' created!", filename));
        notification::send_notification(format!("Gif '{}' created!", filename).as_str());
      }
      Err(error) => panic!("Failed to create gif: {}", error),
    }
  });

  collector_thread.join().unwrap();
  writer_thread.join().unwrap();
}

pub fn save_raw(frames: Vec<frame::ClipFrame>, dimensions: (usize, usize)) {
  let buffer = match frame::frames_to_binary(frames, dimensions) {
    Ok(buffer) => buffer,
    Err(error) => panic!("Failed to convert frames to binary: {}", error),
  };

  println!("Save raw buffer size: {}B", buffer.len());
}
