mod logger;
mod clip;

use notify_rust::Notification;

fn main() {
    send_notification("Clipping 2nd screen");
    clip::clip_screen(0);
}

fn send_notification(message: &str) {
    match Notification::new().summary(message).show() {
        Ok(_) => {}
        Err(_) => {}
    };
}