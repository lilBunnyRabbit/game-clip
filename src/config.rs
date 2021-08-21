use gifski;

pub struct Config {
  pub quality: u8, // max 100
  pub fast: bool,
  pub repeat: gifski::Repeat,
  pub fps: u32,
  pub duration: usize, // in seconds
  pub width: u32,
  pub height: u32,
}

pub fn get() -> Config {
  // Todo: config file
  return Config {
    quality: 100,
    fast: true,
    repeat: gifski::Repeat::Infinite,
    fps: 60,
    duration: 1,
    width: 640,
    height: 360
  }
}
