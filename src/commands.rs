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

use super::{Error, ImageBuffer};

/// Horizontal alignment.
#[derive(Copy, Clone, Debug, PartialEq)]
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

    /// Print a greyscale image
    Image(image::ImageBuffer<image::Luma<u8>, Vec<u8>>),
}

impl Command {
    /// Return the command as raw bytes which can be sent to a POS printer.
    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(match self {
            Command::Init => vec![0x1b, b'@'],
            Command::Cut => vec![0x1b, b'i'],
            Command::Underline(state) => vec![0x1b, b'-', if *state { 1 } else { 0 }],
            Command::Emphasize(state) => vec![0x1b, b'E', if *state { 0xFF } else { 0 }],
            Command::DoubleStrike(state) => vec![0x1b, b'G', if *state { 0xFF } else { 0 }],
            Command::Reverse(state) => vec![0x1b, b'B', if *state { 0xFF } else { 0 }],
            Command::Justification(alignment) => vec![0x1b, b'a', *alignment as u8],
            Command::Feed(count) => vec![0x1b, b'd', *count],
            Command::Speed(speed) => vec![0x1d, 0x28, 0x4b, 0x02, 0x00, 0x32, speed % 9],
            Command::Image(img) => {
                let buf: ImageBuffer = (img.clone()).try_into()?;

                let [w1, w2] = buf.width.to_le_bytes();
                let [h1, h2] = buf.height.to_le_bytes();

                [0x1d, 0x76, 0x30, 0x00, w1, w2, h1, h2]
                    .iter()
                    .chain(&buf.pixels)
                    .copied()
                    .collect()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_encoding_of {
        ($name:ident, $ref:expr, || $block:block) => {
            #[test]
            fn $name() {
                let cmd: Command = $block;
                assert_eq!(&$ref, &cmd.as_bytes().unwrap()[..]);
            }
        };
    }

    // "Easy" commands
    test_encoding_of!(encode_init, [0x1b, 0x40], || { Command::Init });
    test_encoding_of!(encode_cut, [0x1b, 0x69], || { Command::Cut });

    // Underline command
    test_encoding_of!(encode_underline_false, [0x1b, 0x2d, 0x00], || {
        Command::Underline(false)
    });
    test_encoding_of!(encode_underline_true, [0x1b, 0x2d, 0x01], || {
        Command::Underline(true)
    });

    // Emphasize command
    test_encoding_of!(encode_emphasize_false, [0x1b, 0x45, 0x00], || {
        Command::Emphasize(false)
    });
    test_encoding_of!(encode_emphasize_true, [0x1b, 0x45, 0xFF], || {
        Command::Emphasize(true)
    });

    // DoubleStrike command
    test_encoding_of!(encode_double_strike_false, [0x1b, 0x47, 0x00], || {
        Command::DoubleStrike(false)
    });
    test_encoding_of!(encode_double_strike_true, [0x1b, 0x47, 0xFF], || {
        Command::DoubleStrike(true)
    });

    // Reverse command
    test_encoding_of!(encode_reverse_false, [0x1b, 0x42, 0x00], || {
        Command::Reverse(false)
    });
    test_encoding_of!(encode_reverse_true, [0x1b, 0x42, 0xFF], || {
        Command::Reverse(true)
    });

    // Justification command
    test_encoding_of!(encode_justify_left, [0x1b, 0x61, 0x00], || {
        Command::Justification(Alignment::Left)
    });
    test_encoding_of!(encode_justify_right, [0x1b, 0x61, 0x02], || {
        Command::Justification(Alignment::Right)
    });
    test_encoding_of!(encode_justify_center, [0x1b, 0x61, 0x01], || {
        Command::Justification(Alignment::Center)
    });

    test_encoding_of!(feed_0, [0x1b, 0x64, 0x00], || { Command::Feed(0) });
    test_encoding_of!(feed_10, [0x1b, 0x64, 0x0a], || { Command::Feed(10) });

    test_encoding_of!(speed_1, [0x1d, 0x28, 0x4b, 0x02, 0x00, 0x32, 0x01], || {
        Command::Speed(1)
    });
    test_encoding_of!(speed_8, [0x1d, 0x28, 0x4b, 0x02, 0x00, 0x32, 0x08], || {
        Command::Speed(8)
    });

    // currently this is modulo
    test_encoding_of!(speed_9, [0x1d, 0x28, 0x4b, 0x02, 0x00, 0x32, 0x00], || {
        Command::Speed(9)
    });

    // TODO: test image encoding here
}

// vim: foldmethod=marker
