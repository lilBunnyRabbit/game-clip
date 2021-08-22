use crate::utils;

use gifski;
use std::io::prelude::*;
use yaml_rust::YamlLoader;
use utils::logger;

#[derive(Debug)]
pub struct Config {
  pub quality: u8, // max 100
  pub fast: bool, // gifski fast
  pub repeat: gifski::Repeat, // gifski repeat
  pub fps: u32, // max fps
  pub duration: usize, // in seconds
  pub width: u32, // gif width
  pub height: u32, // gif height
  pub path: String, // gif path
  pub display: usize // Selected display. 0 primary, 1 secondary, ...
}

static QUALITY: u8 = 100;
static FAST: bool = true;
static REPEAT: gifski::Repeat = gifski::Repeat::Infinite;
static FPS: u32 = 60;
static DURATION: usize = 3;
static WIDTH: u32 = 640;
static HEIGHT: u32 = 360;
static PATH: &str = "./tmp/";
static DISPLAY: usize = 0;

pub fn get() -> Config {
  match std::fs::read_to_string("./config.yaml") {
    Ok(string_data) => {
      let docs = YamlLoader::load_from_str(string_data.as_str()).unwrap();
      let doc = &docs[0];

      let config = Config {
        quality: doc["quality"].as_i64().unwrap() as u8,
        fast: FAST,
        repeat: REPEAT,
        fps: doc["fps"].as_i64().unwrap() as u32,
        duration: doc["duration"].as_i64().unwrap() as usize,
        width: doc["width"].as_i64().unwrap() as u32,
        height: doc["height"].as_i64().unwrap() as u32,
        path: doc["path"].as_str().unwrap().to_string(),
        display: doc["display"].as_i64().unwrap() as usize,
      };

      logger::info(format!("{:?}", config));
      return config;
    }

    Err(_) => {
      let mut file = std::fs::File::create("./config.yaml").unwrap();
      writeln!(file, "quality: {}", QUALITY);
      writeln!(file, "fps: {}", FPS);
      writeln!(file, "duration: {}", DURATION);
      writeln!(file, "width: {}", WIDTH);
      writeln!(file, "height: {}", HEIGHT);
      writeln!(file, "path: {}", PATH);
      writeln!(file, "display: {}", DISPLAY);

      let config = Config {
        quality: QUALITY,
        fast: FAST,
        repeat: REPEAT,
        fps: FPS,
        duration: DURATION,
        width: WIDTH,
        height: HEIGHT,
        path: PATH.to_string(),
        display: DISPLAY
      };

      logger::info(format!("New {:?}", config));
      return config;
    }
  }
}
