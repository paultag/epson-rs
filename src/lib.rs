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

#![deny(missing_docs)]

//! This crate implements support for communicating with the Epson brand of
//! thermal POS printer.

/// Horizontal alignment.
#[repr(u8)]
pub enum Alignment {
    /// Align to the leftmost edge.
    Left = 0,

    /// Align to the rightmost edge.
    Right = 2,

    /// Center the text within the printable region.
    Center = 1,
}

/// All commands that can be encoded to control an Epson printer.
pub enum Command {
    /// Initiaize the printer.
    Init,

    /// If true, underline the printed text following. If false, remove
    /// text decoration.
    Underline(bool),

    /// If true, emphasize the printed text following. If false, remove
    /// text decoration.
    Emphasize(bool),

    /// If true, double strike the printed text following. If false, remove
    /// text decoration.
    DoubleStrike(bool),

    /// If true, invert the color of the the printed text following. If false,
    /// remove text decoration.
    Reverse(bool),

    /// Align the text to follow accoridng to the specified horizontal
    /// text alignment.
    Justification(Alignment),

    /// Set the print speed.
    Speed(u8),

    /// Cut the thermal printer.
    Cut,

    /// Feed the specified number of lines.
    Feed(u8),

    /// Reverse-feed the specified number of lines.
    ReverseFeed(u8),
}

impl Command {
    /// Return the command as raw bytes which can be sent to a POS printer.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Command::Init => vec![0x1b, 0x40],
            _ => vec![],
        }
    }
}

// BarcodeHeight(u8),
// Barcode(Vec<u8>),
// Image(...)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        assert_eq!(&[0x1b, 0x40], &Command::Init.as_bytes()[..]);
    }
}

// vim: foldmethod=marker
