use futures_util::{SinkExt, StreamExt};
use image::codecs::jpeg::JpegEncoder;
use image::{ImageBuffer, Rgb};
use scrap::{Capturer, Display};
use std::io::Cursor;
use tokio::io::ErrorKind::WouldBlock;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9002").await?;
    println!("WebSocket server started on ws://0.0.0.0:9002");

    while let Ok((stream, _)) = listener.accept().await {
        handle_connection(stream).await?;
    }
    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut ws_sender, _) = ws_stream.split();

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
                let mut jpeg_data = Vec::new();
                let mut cursor = Cursor::new(&mut jpeg_data);
                let mut encoder = JpegEncoder::new_with_quality(&mut cursor, 85);
                encoder.encode_image(&img)?;
                ws_sender.send(Message::Binary(jpeg_data)).await?;

                tokio::time::sleep(std::time::Duration::from_millis(1000 / 30)).await;
            }
            Err(ref e) if e.kind() == WouldBlock => {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}
