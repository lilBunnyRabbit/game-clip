#![allow(dead_code)]
use chrono::prelude::Utc;

pub fn info<S: AsRef<str>>(msg: S) {
  log("INFO", msg)
}
pub fn warn<S: AsRef<str>>(msg: S) {
  log("WARN", msg)
}
pub fn error<S: AsRef<str>>(msg: S) {
  log("ERROR", msg)
}
pub fn debug<S: AsRef<str>>(msg: S) {
  log("DEBUG", msg)
}

fn log<S: AsRef<str>>(typ: &str, msg: S) {
  println!("[{}] {:<5} | {}", Utc::now().format("%F %T"), typ, msg.as_ref());
}
