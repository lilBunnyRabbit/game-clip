mod capture;
mod clip;
mod config;
mod frame;
mod utils;

use scrap;
use utils::logger;

fn main() {
  let config = config::get();
  let display = match get_display(config.display) {
    Ok(display) => display,
    Err(error) => panic!("Failed to get the display: {}", error),
  };

  let capturer = scrap::Capturer::new(display).unwrap();
  let dimensions = (capturer.width(), capturer.height());

  capture::capture_frames(capturer, dimensions);
}

fn get_display(display_index: usize) -> Result<scrap::Display, &'static str> {
  let display: scrap::Display;

  if display_index == 0 {
    display = match scrap::Display::primary() {
      Ok(display) => display,
      Err(_) => return Err("Failed to fetch primary display")
    }
  } else {
    let mut displays = scrap::Display::all().unwrap();

    if displays.len() < display_index + 1 {
      return Err("Display doesn't exist");
    }

    display = displays.remove(display_index);
  }

  logger::info(format!(
    "Selected display: {}x{}{}",
    display.width(),
    display.height(),
    if display_index == 0 { " (primary)" } else { "" }
  ));

  return Ok(display);
}
