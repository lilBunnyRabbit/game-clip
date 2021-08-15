use crate::config;
use crate::logger;
use crate::utils;

use gifski;
use imgref;
use rgb;
use scrap;
use std::{fs::File, thread, time::SystemTime};

pub struct ClipFrame {
  pub frame: Vec<u8>,
  pub delay: f64,
}

impl Clone for ClipFrame {
  fn clone(&self) -> Self {
    ClipFrame {
      frame: self.frame.clone(),
      delay: self.delay.clone(),
    }
  }
}

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

pub fn get_display(display_index: usize) -> Result<scrap::Display, &'static str> {
  let mut displays = scrap::Display::all().unwrap();

  if displays.len() < display_index + 1 {
    return Err("Display doesn't exist");
  }

  let display = displays.remove(display_index);
  logger::info(format!(
    "Selected display: {}x{}",
    display.width(),
    display.height()
  ));

  return Ok(display);
}

pub fn save_gif(frames: Vec<ClipFrame>, dimensions: (usize, usize)) {
  let (mut collector, writer) = init_gifski(dimensions);

  let mut timestamp: f64 = 0.0;

  let collector_thread = thread::spawn(move || {
    for (i, frame) in frames.iter().enumerate() {
      if i != 0 {
        timestamp += frame.delay;
      }
      let imgvec = frame_to_imgvec(dimensions.0, dimensions.1, &frame.frame);

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
    let filename = format!("./tmp/{}.gif", now.as_millis());

    let file = File::create(filename.as_str()).unwrap();
    logger::info(format!("Created file '{}'", filename));

    let progress_reporter: &mut dyn gifski::progress::ProgressReporter = &mut WriterProgress {};

    logger::info("Writing frames");
    match writer.write(file, progress_reporter) {
      Ok(_) => {
        logger::info(format!("Gif '{}' created!", filename));
        utils::send_notification(format!("Gif '{}' created!", filename).as_str());
      }
      Err(error) => panic!("Failed to create gif: {}", error),
    }
  });

  collector_thread.join().unwrap();
  writer_thread.join().unwrap();
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

fn frame_to_imgvec(width: usize, height: usize, frame: &Vec<u8>) -> imgref::ImgVec<rgb::RGBA8> {
  let mut rbga_vec: Vec<rgb::RGBA8> = Vec::new();
  let mut i = 0;
  while i < frame.len() {
    rbga_vec.push(rgb::RGBA8 {
      r: frame[i + 2],
      g: frame[i + 1],
      b: frame[i],
      a: frame[i + 3],
    });

    i += 4;
  }

  return imgref::Img::new(rbga_vec, width, height);
}
