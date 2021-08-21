use crate::config;
use crate::logger;
use crate::utils;

use chrono::prelude::{DateTime, Local};
use gifski;
use imgref;
use rgb;
use scrap;
use std::fmt;
use std::io::Error;
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

impl std::fmt::Display for ClipFrame {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{ size: {}, delay: {} }}", self.frame.len(), self.delay)
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

pub fn _save_raw(frames: Vec<ClipFrame>, dimensions: (usize, usize)) -> Result<(), Error> {
  let local: DateTime<Local> = Local::now();
  let _filename = format!("./tmp/{}-test.gc", local.format("%F-%H-%M-%S"));
  // let mut file = File::create(filename)?;

  let mut buffer: Vec<u8> = Vec::new();
  buffer.extend(dimensions.0.to_be_bytes()); // Width 8B
  buffer.extend(dimensions.1.to_be_bytes()); // Height 8B

  let mut timestamp: f64 = 0.0;
  for (i, frame) in frames.iter().enumerate() {
    if i != 0 {
      timestamp += frame.delay;
    }

    logger::info(format!(
      "Added frame {} at {} to buffer ({}MB)",
      i,
      timestamp,
      buffer.len() / 1024 / 1024
    ));

    buffer.extend(timestamp.to_be_bytes()); // Delay 8B
    buffer.extend(&frame.frame); // Frame width * height * 4B
  }

  println!("Buffer size: {}B", buffer.len());
  // file.write_all(&buffer)?;
  // write!(file, "{}{}", dimensions.0, dimensions.1);
  Ok(())
}

fn _clip_frame_to_rgba_string(clip_frame: &ClipFrame, timestamp: f64) -> String {
  let mut string_frame = String::from(format!("{}", timestamp));
  let frame_len = clip_frame.frame.len();
  let mut i = 0;
  while i < frame_len {
    string_frame.push_str(&format!(
      " {} {} {} {}",
      clip_frame.frame[i + 2],
      clip_frame.frame[i + 1],
      clip_frame.frame[i],
      clip_frame.frame[i + 3]
    ));

    i += 4;
  }

  return string_frame;
}
