use epson::{Model, StdWriterExt};
use std::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img = image::io::Reader::open(args[2].clone())?.decode()?;
    let mut stream = TcpStream::connect(args[1].clone())?;
    let mut pos = epson::Writer::open(Model::T20II, &mut stream)?;

    pos.speed(5)?;
    pos.print_image(img.resize(576, 1000000, image::imageops::Lanczos3).into())?;
    pos.feed(5)?;
    pos.cut()?;

    Ok(())
}
