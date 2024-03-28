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

use super::{CharacterSet, Error};

/// Maintained and understood models of Epson Printers.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Model {
    /// Some sort of generic Epson compatable thermal printer. This has
    /// a generic set of capabilities and some safe defaults. This may
    /// be overly constrained if you have a model that is not in this
    /// list.
    Generic,

    /// TM-T20II (such as the Ethernet-based M267D) Epson brand thermal
    /// printer. This is the printer I use to test with and likely the
    /// best supported one at the moment.
    T20II,

    /// TM-T30II Epson brand thermal printer.
    T30II,
}

impl Model {
    /// Return the maximum number of pixels that is wise to send to the printer.
    ///
    /// This is in *pixels*, not bytes, and images from the image crate can
    /// be compared to this value.
    pub fn get_max_image_width(&self) -> usize {
        match self {
            // Lower, but safe, default.
            Model::Generic => 512,

            // the T20II has 12 pixels per column, 48 columns, so 576
            // pixels.
            Model::T20II => 576,

            // the T30II has 12 pixels per column, 48 columns, so 576
            // pixels.
            Model::T30II => 576,
        }
    }

    /// Return the level of support for a specific character set.
    pub fn supports_character_set(&self, c: CharacterSet) -> bool {
        match c {
            CharacterSet::Raw => true,
            CharacterSet::Unicode => match self {
                Model::T20II => false,
                Model::T30II => true,
                Model::Generic => false,
            },
        }
    }

    /// Return the number of printable columns in normal text mode. Some models
    /// may support different column capacities. When the time comes, add
    /// a mode inside the enum.
    pub fn get_columns(&self) -> usize {
        match self {
            Model::Generic => 48,
            Model::T20II => 48,
            Model::T30II => 48,
        }
    }

    /// Check to ensure that the Image is printable.
    pub(crate) fn check_image(&self, img: &image::GrayImage) -> Result<(), Error> {
        let (width, _) = img.dimensions();

        if width
            > self
                .get_max_image_width()
                .try_into()
                .expect("internal error with this model of printer; please file a bug")
        {
            // try and ensure we don't print trash by checking that
            // the image is well-formed.
            return Err(Error::ImageTooLarge);
        }

        Ok(())
    }
}

// vim: foldmethod=marker
