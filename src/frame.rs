use crate::logger;

use imgref;
use rgb;
use std::fmt;
use std::io::Error;

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

pub fn frame_to_imgvec(frame: &Vec<u8>, dimensions: (usize, usize)) -> imgref::ImgVec<rgb::RGBA8> {
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

  return imgref::Img::new(rbga_vec, dimensions.0, dimensions.1);
}

pub fn frames_to_binary(
  frames: Vec<ClipFrame>,
  dimensions: (usize, usize),
) -> Result<Vec<u8>, Error> {
  // [width (8B)]
  // [height (8B)]
  // frames * [delay (8B) | BGRA frame (width * height * 4B)]

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

  Ok(buffer)
}
