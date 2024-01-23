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
#![feature(trait_alias)]

//! This crate implements support for communicating with the Epson brand of
//! thermal POS printer.

mod commands;
mod epson_image;
mod models;
mod write;

#[cfg(feature = "tokio")]
mod async_tokio;

pub use commands::{Alignment, Command};
use epson_image::ImageBuffer;
pub use models::Model;
pub use write::Writer;

#[cfg(feature = "tokio")]
pub use async_tokio::AsyncWriter;

/// Possible error states that we can get returned from the crate
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// This is returned when an image is not aligned to 8 in the width
    /// direction. This is needed because an image's row of pixels is
    /// packed into uint8s, each bit is a single pixel's true/false state.
    ImageNot8BitAligned,

    /// This error is returned when an Image is larger than the encoding
    /// scheme can support (u16) or if the Image is wider than the model
    /// supports.
    ImageTooLarge,

    /// This is returned if the requested function is not supported by the
    /// configured Model.
    Unsupported,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

// vim: foldmethod=marker
