use epson::{Alignment, Model};
use std::error::Error;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let stream = TcpStream::connect(args[1].clone()).await?;
    let mut pos = epson::AsyncWriter::open(Model::T20II, Box::new(stream)).await?;

    pos.speed(5).await?;

    pos.justify(Alignment::Left).await?;
    pos.write_all(b"Hello,\n").await?;
    pos.justify(Alignment::Center).await?;
    pos.write_all(b"there,\n").await?;
    pos.justify(Alignment::Right).await?;
    pos.write_all(b"World!\n").await?;
    pos.justify(Alignment::Left).await?;

    let mut img: image::GrayImage = image::ImageBuffer::new(576, 64);
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, image::Luma([0]));
        }
    }
    pos.print_image(img).await?;

    pos.feed(5).await?;
    pos.cut().await?;

    Ok(())
}
