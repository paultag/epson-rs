use epson::{Model, StdWriterExt};
use std::{io::Write, net::TcpStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut stream = TcpStream::connect(args[1].clone())?;
    let mut pos = epson::Writer::open(Model::T30II, &mut stream)?;

    pos.set_unicode()?;

    pos.speed(5)?;
    pos.write_all("hello there testing one two 53°".as_bytes())?;
    pos.feed(10)?;
    pos.cut()?;

    Ok(())
}
