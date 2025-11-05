use image::codecs::jpeg::JpegEncoder;
use image::{ImageBuffer, Rgb};
use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    let width = capturer.width();
    let height = capturer.height();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width as u32, height as u32);
                let stride = width * 4;
                for y in 0..height {
                    let row_start = y * stride;
                    for x in 0..width {
                        let i = row_start + x * 4;
                        if i + 3 < frame.len() {
                            let pixel = &frame[i..i + 4];
                            img.put_pixel(x as u32, y as u32, Rgb([pixel[2], pixel[1], pixel[0]]));
                        }
                    }
                }
                let mut file = File::create("screenshot.jpg")?;
                let mut encoder = JpegEncoder::new_with_quality(&mut file, 95); // 95 — high quality(0–100)
                encoder.encode_image(&img)?;

                break Ok(());
            }
            Err(ref e) if e.kind() == WouldBlock => {
                thread::sleep(time::Duration::from_millis(10));
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}
