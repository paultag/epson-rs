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

use super::{
    Alignment, Barcode, CharacterSet, Command, Error as EpsonError, Model, Writer,
    commands::HriPosition,
};
use std::io::Write;

/// All errors that can be returned from the sync code in the Epson module.
#[derive(Debug)]
pub enum Error {
    /// Raw Epson error.
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
        write!(f, "{self:?}")
    }
}

/// Result-type used by this file.
type Result<T> = std::result::Result<T, Error>;

/// Trait to add std sync epson methods.
pub trait StdWriterExt
where
    Self: Sized,
{
    /// Inner type of the  writer
    type Write: Write;

    /// Create a new Writer
    fn open(model: Model, w: Self::Write) -> Result<Self>;

    /// initialize the epson printer
    fn init(&mut self) -> Result<()>;

    /// Set unicode mode on the printer, if supported.
    fn set_unicode(&mut self) -> Result<()>;

    /// Set the specific [CharacterSet] to be used on bytes sent to the
    /// printer. Some models do not support sets other than `Raw`, so
    /// check your specific printer model.
    fn character_set(&mut self, c: CharacterSet) -> Result<()>;

    /// cut the printer paper
    fn cut(&mut self) -> Result<()>;

    /// If true, text printed after this command will be underlined. If false,
    /// it will remove an underline if one was set.
    fn underline(&mut self, state: bool) -> Result<()>;

    /// If true, emphasize the text printed after this command. if false,
    /// remove emphasis on the text.
    fn emphasize(&mut self, state: bool) -> Result<()>;

    /// If true, reverse the color of the text printed after this command.
    /// if false, return the colors to normal.
    fn reverse(&mut self, state: bool) -> Result<()>;

    /// If true, double-strike the text printed after this command.
    /// If false, remove the double-strike.
    fn double_strike(&mut self, state: bool) -> Result<()>;

    /// Set the horizontal justification of the text printed after this
    /// command.
    fn justify(&mut self, alignment: Alignment) -> Result<()>;

    /// Feed the specified number of lines out of the printer.
    fn feed(&mut self, count: u8) -> Result<()>;

    /// Set the printer speed to the provided value.
    fn speed(&mut self, speed: u8) -> Result<()>;

    /// Set the print position of HRI (Human Readable Interpretation) characters for barcodes.
    fn set_hri_position(&mut self, position: HriPosition) -> Result<()>;

    /// Print a greyscale image.
    ///
    /// Currently, this image must have a width that's 8 bit aligned,
    /// and the size may not be larger than a uint16 in height. The
    /// width of the image is constrained by the underling printer model
    /// provided to `Self::open`.
    fn print_image(&mut self, img: image::GrayImage) -> Result<()>;

    /// Print a grayscale image, without any model checks. This will let you
    /// do all sorts of invalid things. Don't use this if you can avoid it,
    /// it may result in trash being printed.
    fn print_image_unchecked(&mut self, img: image::GrayImage) -> Result<()>;

    /// Print a barcode.
    ///
    /// The barcode will be printed according to the currently set HRI position.
    /// Use `set_hri_position` to control the position of the human-readable text.
    fn print_barcode(&mut self, barcode: Barcode) -> Result<()>;

    /// Send a raw command to the Epson printer.
    fn write_command(&mut self, cmd: Command) -> Result<()>;
}

impl<WriteT> StdWriterExt for Writer<WriteT>
where
    WriteT: Write,
{
    type Write = WriteT;

    /// Create a new Writer
    fn open(model: Model, w: WriteT) -> Result<Self> {
        let mut r = Self { w, model };
        r.init()?;
        Ok(r)
    }

    /// initialize the epson printer
    fn init(&mut self) -> Result<()> {
        self.write_command(Command::Init)
    }

    /// Set unicode mode on the printer, if supported.
    fn set_unicode(&mut self) -> Result<()> {
        self.character_set(CharacterSet::Unicode)
    }

    /// Set the specific [CharacterSet] to be used on bytes sent to the
    /// printer. Some models do not support sets other than `Raw`, so
    /// check your specific printer model.
    fn character_set(&mut self, c: CharacterSet) -> Result<()> {
        if !self.model.supports_character_set(c) {
            return Err(EpsonError::Unsupported.into());
        }

        self.write_command(Command::CharacterSet(c))
    }

    /// cut the printer paper
    fn cut(&mut self) -> Result<()> {
        self.write_command(Command::Cut)
    }

    /// If true, text printed after this command will be underlined. If false,
    /// it will remove an underline if one was set.
    fn underline(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Underline(state))
    }

    /// If true, emphasize the text printed after this command. if false,
    /// remove emphasis on the text.
    fn emphasize(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Emphasize(state))
    }

    /// If true, reverse the color of the text printed after this command.
    /// if false, return the colors to normal.
    fn reverse(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Reverse(state))
    }

    /// If true, double-strike the text printed after this command.
    /// If false, remove the double-strike.
    fn double_strike(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::DoubleStrike(state))
    }

    /// Set the horizontal justification of the text printed after this
    /// command.
    fn justify(&mut self, alignment: Alignment) -> Result<()> {
        self.write_command(Command::Justification(alignment))
    }

    /// Feed the specified number of lines out of the printer.
    fn feed(&mut self, count: u8) -> Result<()> {
        self.write_command(Command::Feed(count))
    }

    /// Set the printer speed to the provided value.
    fn speed(&mut self, speed: u8) -> Result<()> {
        self.write_command(Command::Speed(speed))
    }

    /// Set the print position of HRI (Human Readable Interpretation) characters for barcodes.
    fn set_hri_position(&mut self, position: HriPosition) -> Result<()> {
        self.write_command(Command::SetHriPosition(position))
    }

    /// Print a greyscale image.
    ///
    /// Currently, this image must have a width that's 8 bit aligned,
    /// and the size may not be larger than a uint16 in height. The
    /// width of the image is constrained by the underling printer model
    /// provided to `Self::open`.
    fn print_image(&mut self, img: image::GrayImage) -> Result<()> {
        self.model.check_image(&img)?;
        self.print_image_unchecked(img)
    }

    /// Print a grayscale image, without any model checks. This will let you
    /// do all sorts of invalid things. Don't use this if you can avoid it,
    /// it may result in trash being printed.
    fn print_image_unchecked(&mut self, img: image::GrayImage) -> Result<()> {
        self.write_command(Command::Image(img))
    }

    /// Print a barcode.
    ///
    /// The barcode will be printed according to the currently set HRI position.
    /// Use `set_hri_position` to control the position of the human-readable text.
    fn print_barcode(&mut self, barcode: Barcode) -> Result<()> {
        self.write_command(Command::Barcode(barcode))
    }

    /// Send a raw command to the Epson printer.
    fn write_command(&mut self, cmd: Command) -> Result<()> {
        self.write_all(&cmd.as_bytes()?)?;
        Ok(())
    }
}

impl<WriteT> Write for Writer<WriteT>
where
    WriteT: Write,
{
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.w.write(b)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

// vim: foldmethod=marker
