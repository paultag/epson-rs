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

use super::Error;

/// INTERNAL only type to represent an image buffer. The API is only
/// the standard crate Image type(s).
///
/// This can only represent a black or white pixel; every u8 represents
/// 8 pixels, 8 pixels in a row.
pub(crate) struct ImageBuffer {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) pixels: Vec<u8>,
}

impl TryFrom<image::ImageBuffer<image::Luma<u8>, Vec<u8>>> for ImageBuffer {
    type Error = Error;

    fn try_from(img: image::ImageBuffer<image::Luma<u8>, Vec<u8>>) -> Result<Self, Error> {
        let (mut width, height) = img.dimensions();

        if width % 8 != 0 {
            width += 8 - (width % 8);
        }

        let mut pixels = vec![];

        for y in 0..height {
            for x in (0..width).step_by(8) {
                let mut block: u8 = 0;
                for bit in 0..8 {
                    if let Some(pixel) = img.get_pixel_checked(x + bit, y)
                        && pixel.0[0] <= 128
                    {
                        block |= 1 << (7 - bit)
                    }
                }
                pixels.push(block);
            }
        }

        let width = width / 8;
        let width: u16 = width.try_into().map_err(|_| Error::ImageTooLarge)?;
        let height: u16 = height.try_into().map_err(|_| Error::ImageTooLarge)?;

        Ok(ImageBuffer {
            width,
            height,
            pixels,
        })
    }
}

// vim: foldmethod=marker
