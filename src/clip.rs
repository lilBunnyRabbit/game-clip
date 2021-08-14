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
    println!("\t- progress increase");
    true
  }
  fn done(&mut self, msg: &str) {
    println!("\t- progress done: {}", msg);
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

  let (collector, writer) = init_gifski(dimensions);

  add_frames_to_collector(frames, collector, dimensions);
  save_gif(writer);
}

fn get_display(display_index: usize) -> Result<scrap::Display, &'static str> {
  let mut displays = scrap::Display::all().unwrap();

  print!("Displays: [");
  for (i, display) in displays.iter().enumerate() {
    print!("{}x{}", display.width(), display.height());
    if displays.len() != i + 1 {
      print!(", ");
    }
  }
  println!("]");

  if displays.len() < display_index + 1 {
    return Err("Display doesn't exist");
  }

  // let display = scrap::Display::primary().expect("Couldn't find primary display.");
  let display = displays.remove(display_index);
  println!("Selected display: {}x{}", display.width(), display.height());

  return Ok(display);
}

fn capture_frames(mut capturer: scrap::Capturer) -> Vec<Vec<u8>> {
  let fps = 30;
  let one_frame = Duration::new(1, 0) / fps;

  let mut frames: Vec<Vec<u8>> = Vec::new();
  let start = Instant::now();

  println!("Capturing frames:");
  loop {
    match capturer.frame() {
      Ok(frame) => {
        frames.push(frame.to_vec());
        println!("\t- frame {}", frames.len());

        if frames.len() == fps as usize {
          break;
        }
      }
      Err(ref e) if e.kind() == WouldBlock => {
        thread::sleep(one_frame);
      }
      Err(_) => break,
    }
  }

  println!(
    "Finished capturing in {}s...",
    start.elapsed().as_secs_f32()
  );
  return frames;
}

fn init_gifski(dimensions: (usize, usize)) -> (gifski::Collector, gifski::Writer) {
  let settings = gifski::Settings {
    width: Some(dimensions.0 as u32),
    height: Some(dimensions.1 as u32),
    quality: 100,
    fast: true,
    repeat: gifski::Repeat::Infinite,
  };

  println!("Gifski init");
  return gifski::new(settings).unwrap();
}

fn add_frames_to_collector(frames: Vec<Vec<u8>>, mut collector: gifski::Collector, dimensions: (usize, usize)) {
  println!("Adding frames to collector:");
  for (i, frame) in frames.iter().enumerate() {
    // if i > 4 { break; }

    let timestamp: f64 = i as f64 * 0.05;
    print!("\t- frame {}: {}, ", i, timestamp);

    let imgvec = frame_to_imgvec(dimensions.0, dimensions.1, frame);
    print!("imgvec, ");

    match collector.add_frame_rgba(i, imgvec, 0.0) {
      Ok(_) => print!("collector, "),
      Err(error) => panic!("Err adding frame {}", error),
    }

    println!("done");
  }

  drop(collector);
  println!("Dropped collector");
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

fn save_gif(writer: gifski::Writer) {
  let now: std::time::Duration = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("Couldn't get Epoch time");
  let filename = format!("./tmp/{}.gif", now.as_millis());

  let file = File::create(filename.as_str()).unwrap();
  println!("Created file '{}'", filename);

  let progress_reporter: &mut dyn gifski::progress::ProgressReporter = &mut SomeProgress {};

  println!("Creating gif:");
  match writer.write(file, progress_reporter) {
    Ok(_) => println!("Gif '{}' created!", filename),
    Err(error) => println!("Failed to create gif: {}", error),
  }
}
