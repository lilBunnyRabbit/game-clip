use captrs::{ Capturer, Bgr8 };
use notify_rust::Notification;
use image::{ RgbaImage, Rgba };
use std::fs::File;
use std::io::prelude::*;
use scrap;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    scrap_test();
    // captrs_test();
}

fn scrap_test() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    
    let display = scrap::Display::primary().expect("Couldn't find primary display.");
    let mut capturer = scrap::Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        println!("Captured! Saving...");

        println!("{}", buffer[0]);
        image_test2((w, h), buffer);
        break;
    }
}

fn image_test2(dimensions: (usize, usize), data: scrap::Frame) {
    let mut img = RgbaImage::new(dimensions.0 as u32, dimensions.1  as u32);
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    let mut i = 0;
    while i < data.len() {
        let rgba = [data[i + 2], data[i + 1], data[i], data[i + 3]];
        img.put_pixel(x, y, Rgba(rgba));

        x += 1;
        if x >= dimensions.0 as u32 {
            x = 0;
            y += 1;
        }

        i += 4;
    }

    img.save("output.png").unwrap();
}

fn save_image2(dimensions: (usize, usize), data: scrap::Frame) -> std::io::Result<()> {
    let mut string_data = String::from("P3\n");
    string_data.push_str(format!("{} {}\n", dimensions.0, dimensions.1).as_str());

    let stride = data.len() / dimensions.1;
    for i in 0..(dimensions.0 * dimensions.1) {
        string_data.push_str(format!("{} {} {} ", data[i], data[i + 1], data[i + 2]).as_str());
    }

    string_data.push_str("\n");
    let mut file = File::create("foo.ppm")?;
    file.write_all(&mut string_data.as_bytes())?;
    Ok(())
}

fn captrs_test() {
    match Capturer::new(1) {
        Ok(capturer) => match_capturer(capturer),
        Err(error) => panic!("Failed to create capturer: {:?}", error),
    };
}

fn match_capturer(mut capturer: Capturer) {
    match capturer.capture_store_frame() {
        Ok(_) => {
            match capturer.capture_store_frame() {
                Ok(_) => match_capturer_store_frame(capturer),
                Err(error) => panic!("Failed to capture frame: {:?}", error)
            }
        },
        Err(error) => panic!("Failed to capture frame: {:?}", error)
    }
}

fn match_capturer_store_frame(capturer: Capturer) {
    // match capturer.capture_frame() {
    //     Ok(data) => {
    //         for f in 1000..1010 {
    //             print!("{:?} ", data[f]);
    //         }
    //         println!("");
    //         image_test(capturer.geometry(), data);
    //     },
    //     Err(_error) => {}
    // }

    match capturer.get_stored_frame() {
        Some(data) => {
            send_notification("Frame Stored");
            for f in 1000..1010 {
                print!("{:?} ", data[f]);
            }
            println!("");
            
            match save_image(capturer.geometry(), data) {
                Ok(_) => {},
                Err(error) => panic!("Failed to save_image: {:?}", error)
            }
        },
        None => panic!("Failed to get stored frame")
    }
}

fn save_image(dimensions: (u32, u32), data: &[Bgr8]) -> std::io::Result<()> {
    let mut string_data = String::from("P3\n");
    string_data.push_str(format!("{} {}\n", dimensions.0, dimensions.1).as_str());
    for x in 0..=dimensions.0 - 1 {
        for y in 0..=dimensions.1 - 1 {
            let pixel = data[(x + y) as usize];
            string_data.push_str(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_str());
        }
    }

    string_data.push_str("\n");
    let mut file = File::create("foo.ppm")?;
    file.write_all(&mut string_data.as_bytes())?;
    Ok(())
}

fn image_test(dimensions: (u32, u32), data: &[Bgr8]) {
    let mut img = RgbaImage::new(dimensions.0, dimensions.1);

    for x in 0..=dimensions.0 - 1 {
        for y in 0..=dimensions.1 - 1 {
            let pixel = data[(x + y) as usize];
            img.put_pixel(x, y, Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
        }
    }

    img.save("output.png").unwrap();
}

/*
P3           # "P3" means this is a RGB color image in ASCII
3 2          # "3 2" is the width and height of the image in pixels
255          # "255" is the maximum value for each color
# The part above is the header
# The part below is the image data: RGB triplets
255   0   0  # red
  0 255   0  # green
  0   0 255  # blue
255 255   0  # yellow
255 255 255  # white
  0   0   0  # black
*/

// fn image_test(dimensions: (u32, u32), data: &[Bgr8]) {
//     let mut img = RgbaImage::new(dimensions.0, dimensions.1);

//     for x in 0..=dimensions.0 - 1 {
//         for y in 0..=dimensions.1 - 1 {
//             let pixel = data[(x + y) as usize];
//             img.put_pixel(x, y, Rgba([pixel.r, pixel.g, pixel.b, pixel.a]));
//         }
//     }

//     img.save("output.png").unwrap();
// }

fn send_notification(message: &str) {
    match Notification::new()
    .summary(message)
    .show() {
        Ok(_) => {},
        Err(_) => {}
    };
}
