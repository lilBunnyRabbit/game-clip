use crate::logger;

use gifski;
use imgref;
use rgb;
use scrap;
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

pub struct ClipSettings {
  pub quality: u8,
  pub fast: bool,
  pub repeat: gifski::Repeat,
  pub fps: usize,
  pub duration: usize,
  pub size: usize,
}

static FPS: usize = 30;
pub static SETTINGS: ClipSettings = ClipSettings {
  quality: 100,
  fast: true,
  repeat: gifski::Repeat::Infinite,
  fps: FPS,
  duration: 5,
  size: 1,
};

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

pub fn get_frame_time() -> std::time::Duration {
  return std::time::Duration::new(1, 0) / FPS as u32;
}

pub fn save_gif(frames: Vec<Vec<u8>>, dimensions: (usize, usize)) {
  let (mut collector, writer) = init_gifski(dimensions);

  let frame_time = get_frame_time();

  let collector_thread = thread::spawn(move || {
    for (i, frame) in frames.iter().enumerate() {
      let timestamp: f64 = i as f64 * frame_time.as_secs_f64();
      logger::info(format!("Adding frame {} at {}, ", i, timestamp));

      let imgvec = frame_to_imgvec(dimensions.0, dimensions.1, frame);

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
      Ok(_) => logger::info(format!("Gif '{}' created!", filename)),
      Err(error) => panic!("Failed to create gif: {}", error),
    }
  });

  collector_thread.join().unwrap();
  writer_thread.join().unwrap();
}

fn init_gifski(_dimensions: (usize, usize)) -> (gifski::Collector, gifski::Writer) {
  let gif_settings = gifski::Settings {
    // width: Some((dimensions.0 / SETTINGS.size) as u32),
    width: Some(640),
    // height: Some((dimensions.1 / SETTINGS.size) as u32),
    height: Some(360),
    quality: SETTINGS.quality,
    fast: SETTINGS.fast,
    repeat: SETTINGS.repeat,
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
