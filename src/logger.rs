#![allow(dead_code)]
use chrono::prelude::Utc;
use std::thread;

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
  let thread_id = thread::current().id();
  println!("[{}] {:<5} | {:?} | {}", Utc::now().format("%F %T"), typ, thread_id, msg.as_ref());
}
