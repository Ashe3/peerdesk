use image::{ImageBuffer, Rgb};
use scrap::{Capturer, Display};
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

                for (i, pixel) in frame.chunks(4).enumerate() {
                    let x = (i % width) as u32;
                    let y = (i / width) as u32;
                    img.put_pixel(x, y, Rgb([pixel[2], pixel[1], pixel[0]]));
                }

                img.save("screenshot.jpg")?;
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
