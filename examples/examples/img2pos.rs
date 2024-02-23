use epson::Model;
use std::{io::Write, net::TcpStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let img = image::io::Reader::open(args[2].clone())?.decode()?;
    let stream = TcpStream::connect(args[1].clone())?;
    let mut pos = epson::Writer::open(Model::T20II, Box::new(stream))?;

    pos.speed(5)?;
    pos.print_image(img.into())?;
    pos.feed(5)?;
    pos.cut()?;

    Ok(())
}
