use notify_rust::Notification;

pub fn send_notification(message: &str) {
  match Notification::new().summary(message).show() {
    Ok(_) => {}
    Err(_) => {}
  };
}