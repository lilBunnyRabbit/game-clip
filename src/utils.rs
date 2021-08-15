use notify_rust::Notification;
use std::mem::transmute;

pub fn send_notification(message: &str) {
  match Notification::new().summary(message).show() {
    Ok(_) => {}
    Err(_) => {}
  };
}

pub fn u16_to_bytes(data: u16) -> [u8; 2] {
  return data.to_be_bytes();
  // let bytes: [u8; 2] = unsafe { transmute(data.to_be()) };
  // println!("{} -> {:?}", data, bytes);
  // return bytes;
}

pub fn f64_to_bytes(data: f64) -> [u8; 8] {
  return data.to_be_bytes();
}
