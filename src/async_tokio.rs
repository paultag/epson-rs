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

#![cfg_attr(docsrs, doc(cfg(feature = "tokio")))]

use super::{Alignment, CharacterSet, Command, Error as EpsonError, HriPosition, Model};
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// All possible errors that can be returned from the AsyncWriter struct.
#[derive(Debug)]
pub enum Error {
    /// Raw Epson error, as defined in [EpsonError].
    Epson(EpsonError),

    /// Underlying Tokio i/o issue.
    Tokio(tokio::io::Error),
}

impl From<EpsonError> for Error {
    fn from(ee: EpsonError) -> Error {
        Error::Epson(ee)
    }
}

impl From<tokio::io::Error> for Error {
    fn from(te: tokio::io::Error) -> Error {
        Error::Tokio(te)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Result alias for the AsyncWriter methods.
type Result<T> = std::result::Result<T, Error>;

/// Write alias for the AsyncWrite W type.
type Write = dyn AsyncWrite + Unpin + Send;

/// Wrapper around a `tokio` [AsyncWrite] handle to write to an Epson printer
/// using a tokio i/o connection such as a TcpStream.
pub struct AsyncWriter {
    w: Box<Write>,
    model: Model,
}

impl AsyncWriter {
    /// Create a new Writer, wrapping the provided `tokio::io::AsyncWrite`.
    pub async fn open(model: Model, w: Box<Write>) -> Result<Self> {
        let mut r = Self { w, model };
        r.init().await?;
        Ok(r)
    }

    /// initialize the epson printer
    async fn init(&mut self) -> Result<()> {
        self.write_command(Command::Init).await
    }

    /// cut the printer paper
    pub async fn cut(&mut self) -> Result<()> {
        self.write_command(Command::Cut).await
    }

    /// Set unicode mode on the printer, if supported.
    pub async fn set_unicode(&mut self) -> Result<()> {
        self.character_set(CharacterSet::Unicode).await
    }

    /// Set the specific [CharacterSet] to be used on bytes sent to the
    /// printer. Some models do not support sets other than `Raw`, so
    /// check your specific printer model.
    pub async fn character_set(&mut self, c: CharacterSet) -> Result<()> {
        if !self.model.supports_character_set(c) {
            return Err(EpsonError::Unsupported.into());
        }
        self.write_command(Command::CharacterSet(c)).await
    }

    /// If true, text printed after this command will be underlined. If false,
    /// it will remove an underline if one was set.
    pub async fn underline(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Underline(state)).await
    }

    /// If true, emphasize the text printed after this command. if false,
    /// remove emphasis on the text.
    pub async fn emphasize(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Emphasize(state)).await
    }

    /// If true, reverse the color of the text printed after this command.
    /// if false, return the colors to normal.
    pub async fn reverse(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::Reverse(state)).await
    }

    /// If true, double-strike the text printed after this command.
    /// If false, remove the double-strike.
    pub async fn double_strike(&mut self, state: bool) -> Result<()> {
        self.write_command(Command::DoubleStrike(state)).await
    }

    /// Set the horizontal justification of the text printed after this
    /// command.
    pub async fn justify(&mut self, alignment: Alignment) -> Result<()> {
        self.write_command(Command::Justification(alignment)).await
    }

    /// Feed the specified number of lines out of the printer.
    pub async fn feed(&mut self, count: u8) -> Result<()> {
        self.write_command(Command::Feed(count)).await
    }

    /// Set the printer speed to the provided value.
    pub async fn speed(&mut self, speed: u8) -> Result<()> {
        self.write_command(Command::Speed(speed)).await
    }

    /// Set the print position of HRI (Human Readable Interpretation) characters for barcodes.
    pub async fn set_hri_position(&mut self, position: HriPosition) -> Result<()> {
        self.write_command(Command::SetHriPosition(position)).await
    }

    /// Print a greyscale image.
    ///
    /// Currently, this image must have a width that's 8 bit aligned,
    /// and the size may not be larger than a uint16 in height. The
    /// width of the image is constrained by the underling printer model
    /// provided to `Self::open`.
    pub async fn print_image(&mut self, img: image::GrayImage) -> Result<()> {
        self.model.check_image(&img)?;
        self.print_image_unchecked(img).await
    }

    /// Print a grayscale image, without any model checks. This will let you
    /// do all sorts of invalid things. Don't use this if you can avoid it,
    /// it may result in trash being printed.
    pub async fn print_image_unchecked(&mut self, img: image::GrayImage) -> Result<()> {
        self.write_command(Command::Image(img)).await
    }

    /// Send a raw command to the Epson printer.
    pub async fn write_command(&mut self, cmd: Command) -> Result<()> {
        self.w.write_all(&cmd.as_bytes()?).await?;
        Ok(())
    }

    /// Write the full buffer `buf` to the underlying socket.
    pub async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.w.write_all(buf).await?;
        Ok(())
    }
}

// vim: foldmethod=marker
