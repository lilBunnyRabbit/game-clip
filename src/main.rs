use notify_rust::Notification;
use image::{ RgbaImage, Rgba };
use scrap;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::{Duration, SystemTime, Instant};

fn main() {
    println!("Hello, world!");
    // scrap_screen();
    scrap_test();
}

fn scrap_test() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    
    let displays = scrap::Display::all().unwrap();
    for (i, display) in displays.iter().enumerate() {
        println!("Display {} [{}x{}]", i + 1, display.width(), display.height());
    }

    // let display = &displays[0];
    let display = scrap::Display::primary().expect("Couldn't find primary display.");
    let mut capturer = scrap::Capturer::new(display).unwrap();
    let (width, height) = (capturer.width(), capturer.height());

    let mut frames: Vec<Vec<u8>> = Vec::new();
    let start = Instant::now();
    loop {
        match capturer.frame() {
            Ok(frame) => {
                frames.push(frame.to_vec());
                println!("Frame {}", frames.len());
                if start.elapsed().as_secs() == 1 {
                    println!("Saving clip with {} frames", frames.len());
                    send_notification(format!("Saving clip with {} frames", frames.len()).as_str());

                    for (i, f) in frames.iter().enumerate() {
                        save_screen(i as u32, width, height, f.to_vec());
                    }

                    break;
                }
            },
            Err(ref e) if e.kind() == WouldBlock => {
                thread::sleep(one_frame);
            },
            Err(_) => { break }
        }
    }
}

fn save_screen(iteration: u32, width: usize, height: usize, data: Vec<u8>) {
    println!("Saving frame {}", iteration);
    let mut img = RgbaImage::new(width as u32, height as u32);
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    let mut i = 0;
    while i < data.len() {
        let rgba = [data[i + 2], data[i + 1], data[i], data[i + 3]];
        img.put_pixel(x, y, Rgba(rgba));

        x += 1;
        if x >= width as u32 {
            x = 0;
            y += 1;
        }

        i += 4;
    }

    let now: std::time::Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Couldn't get Epoch time");
    img.save(format!("./tmp/{}_{}.png", now.as_millis(), iteration)).unwrap();
}

fn send_notification(message: &str) {
    match Notification::new()
    .summary(message)
    .show() {
        Ok(_) => {},
        Err(_) => {}
    };
}
