// {{{ Copyright (c) Paul R. Tagliamonte <paultag@gmail.com>, 2016,2024
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE. }}}

use super::{Alignment, Command, Error as EpsonError, Model};
use std::io::Write;

/// Errors that can be returned from the sync code in the Epson module.
#[derive(Debug)]
pub enum Error {
    /// Raw Epson error
    Epson(EpsonError),

    /// Underlying Tokio i/o issue.
    Io(std::io::Error),
}

impl From<EpsonError> for Error {
    fn from(ee: EpsonError) -> Error {
        Error::Epson(ee)
    }
}

impl From<std::io::Error> for Error {
    fn from(se: std::io::Error) -> Error {
        Error::Io(se)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

///
type Result<T> = std::result::Result<T, Error>;

///
pub struct Writer {
    w: Box<dyn Write>,
    model: Model,
}

impl Writer {
    /// Create a new Writer
    pub fn open(model: Model, w: Box<dyn Write>) -> Result<Self> {
        let mut r = Self { w, model };
        r.init()?;
        Ok(r)
    }

    /// initialize the epson printer
    fn init(&mut self) -> Result<()> {
        self.write_command(Command::Init)
    }

    /// cut the printer paper
    pub fn cut(&mut self) -> Result<()> {
        self.write_command(Command::Cut)
    }

    /// If true, text printed after this command will be underlined. If false,
    /// it will remove an underline if one was set.
    pub fn underline(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Underline(state))
    }

    /// If true, emphasize the text printed after this command. if false,
    /// remove emphasis on the text.
    pub fn emphasize(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Emphasize(state))
    }

    /// If true, reverse the color of the text printed after this command.
    /// if false, return the colors to normal.
    pub fn reverse(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Reverse(state))
    }

    /// If true, double-strike the text printed after this command.
    /// If false, remove the double-strike.
    pub fn double_strike(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::DoubleStrike(state))
    }

    /// Set the horizontal justification of the text printed after this
    /// command.
    pub fn justify(&mut self, alignment: Alignment) -> Result<()> {
        self.write_command(Command::Justification(alignment))
    }

    /// Feed the specified number of lines out of the printer.
    pub fn feed(&mut self, count: u8) -> Result<()> {
        self.write_command(Command::Feed(count))
    }

    /// Set the printer speed to the provided value.
    pub fn speed(&mut self, speed: u8) -> Result<()> {
        self.write_command(Command::Speed(speed))
    }

    /// Print a greyscale image.
    ///
    /// Currently, this image must have a width that's 8 bit aligned,
    /// and the size may not be larger than a uint16 in height. The
    /// width of the image is constrained by the underling printer model
    /// provided to `Self::open`.
    pub fn print_image(&mut self, img: image::GrayImage) -> Result<()> {
        self.model.check_image(&img)?;
        self.print_image_unchecked(img)
    }

    /// Print a grayscale image, without any model checks. This will let you
    /// do all sorts of invalid things. Don't use this if you can avoid it,
    /// it may result in trash being printed.
    pub fn print_image_unchecked(&mut self, img: image::GrayImage) -> Result<()> {
        self.write_command(Command::Image(img))
    }

    /// Send a raw command to the Epson printer.
    fn write_command(&mut self, cmd: Command) -> Result<()> {
        self.write_all(&cmd.as_bytes()?)?;
        Ok(())
    }
}

impl Write for Writer {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.w.write(b)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

// vim: foldmethod=marker
