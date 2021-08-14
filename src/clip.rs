use crate::logger;

use gifski;
use imgref;
use rgb;
use scrap;
use std::{
  fs::File,
  io::ErrorKind::WouldBlock,
  thread,
  time::{Duration, Instant, SystemTime},
};

pub struct SomeProgress {}
impl gifski::progress::ProgressReporter for SomeProgress {
  fn increase(&mut self) -> bool {
    logger::info("Progress increased");
    true
  }
  fn done(&mut self, msg: &str) {
    logger::info(format!("Progress done: {}", msg));
  }
}

pub fn clip_screen(display_index: usize) {
  let display = match get_display(display_index) {
    Ok(display) => display,
    Err(error) => panic!("Failed to get the display: {}", error),
  };

  let capturer = scrap::Capturer::new(display).unwrap();
  let dimensions = (capturer.width(), capturer.height());

  let frames = capture_frames(capturer);

  save_gif(frames, dimensions);
}

fn get_display(display_index: usize) -> Result<scrap::Display, &'static str> {
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

fn capture_frames(mut capturer: scrap::Capturer) -> Vec<Vec<u8>> {
  let fps: u32 = 30;
  let duration: usize = 1 * fps as usize;
  let one_frame = Duration::new(1, 0) / fps;

  let mut frames: Vec<Vec<u8>> = Vec::new();
  let start = Instant::now();

  logger::info("Capturing frames");
  loop {
    match capturer.frame() {
      Ok(frame) => {
        frames.push(frame.to_vec());
        logger::info(format!("Captured frame {}", frames.len()));

        if frames.len() == duration {
          break;
        }
      }
      Err(ref e) if e.kind() == WouldBlock => {
        thread::sleep(one_frame);
      }
      Err(_) => break,
    }
  }

  logger::info(format!(
    "Finished capturing frames in {}s...",
    start.elapsed().as_secs_f32()
  ));
  return frames;
}

fn save_gif(frames: Vec<Vec<u8>>, dimensions: (usize, usize)) {
  let (mut collector, writer) = init_gifski(dimensions);

  let collector_thread = thread::spawn(move || {
    for (i, frame) in frames.iter().enumerate() {
      let timestamp: f64 = i as f64 * 0.03;
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

    let progress_reporter: &mut dyn gifski::progress::ProgressReporter = &mut SomeProgress {};

    logger::info("Writing frames");
    match writer.write(file, progress_reporter) {
      Ok(_) => logger::info(format!("Gif '{}' created!", filename)),
      Err(error) => panic!("Failed to create gif: {}", error),
    }
  });

  collector_thread.join().unwrap();
  writer_thread.join().unwrap();
}

fn init_gifski(dimensions: (usize, usize)) -> (gifski::Collector, gifski::Writer) {
  let settings = gifski::Settings {
    width: Some((dimensions.0 / 2) as u32),
    height: Some((dimensions.1 / 2) as u32),
    quality: 100,
    fast: true,
    repeat: gifski::Repeat::Infinite,
  };

  logger::info("Gifski init");
  return gifski::new(settings).unwrap();
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
