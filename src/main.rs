mod capture;
mod clip;
mod config;
mod utils;

use scrap;
use utils::logger;

fn main() {
  let display = match get_display(0) {
    Ok(display) => display,
    Err(error) => panic!("Failed to get the display: {}", error),
  };

  let capturer = scrap::Capturer::new(display).unwrap();
  let dimensions = (capturer.width(), capturer.height());

  capture::capture_frames(capturer, dimensions);
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