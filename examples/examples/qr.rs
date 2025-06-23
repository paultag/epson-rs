use epson::{Model, StdWriterExt};
use qrcode::QrCode;
use std::{io::Write, net::TcpStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let code = QrCode::new("HACK THE PLANET")?;
    let img = code.render::<image::Luma<u8>>().build();

    let args: Vec<String> = std::env::args().collect();
    let mut stream = TcpStream::connect(args[1].clone())?;
    let mut pos = epson::Writer::open(Model::T20II, &mut stream)?;

    pos.speed(5)?;
    pos.write_all(b"HACK THE PLANET\n")?;
    pos.print_image(img)?;
    pos.feed(5)?;
    pos.cut()?;

    Ok(())
}
