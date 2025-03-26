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
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! The epson crate contains Rust bindings to the Epson Point of Sale (POS)
//! thermal printers' printer format.
//!
//! Currently, this library supports a limited number of commands, and some
//! basic interfaces for both synchronous Rust as well as async
//! Rust through [tokio](https://tokio.rs/), behind the `tokio` feature.
//!
//! Docs can be found on [docs.rs](https://docs.rs/epson/latest/epson/),
//! and information about the latest release can be found on
//! [crates.io](https://crates.io/crates/epson).
//!
//! # Supported Models
//!
//! Specific makes/models of thermal printers will be added as I either
//! get my hands on them, or someone maintains the model for the package.
//! If your make/model isn't supported, you can use
//! [models::Model::Generic].
//!
//! | Model | Type                     | Description                     |
//! | ----- | ------------------------ | ------------------------------- |
//! | T20II | [models::Model::T20II]   | Epson TM-T20II Thermal Printer  |
//! | T30II | [models::Model::T30II]   | Epson TM-T30II Thermal Printer  |
//!
//! # Writing to a `std::io::Write`
//!
//! We can write to a `std::io::Write` traited object (such as a `TcpStream`,
//! but maybe something like a Serial device?), you can use a [Writer] to
//! handle writing commands to the printer.
//!
//! ```rust
//! // IP address of the printer
//! let stream = TcpStream::connect("192.168.0.12:9100").unwrap();
//! let mut pos = epson::Writer::open(Model::T20II, Box::new(stream)).unwrap();
//!
//! pos.speed(5).unwrap();
//! pos.write_all(b"HACK THE PLANET\n").unwrap();
//! pos.feed(5).unwrap();
//! pos.cut().unwrap();
//! ```
//!
//! # Writing to a `tokio::io::AsyncWrite`
//!
//! In addition to the `std::io` support, the `epson` crate also contains
//! `tokio` support to write to a `tokio::io::AsyncWrite` using an
//! [AsyncWriter].
//!
//! This requires the `tokio` feature.
//!
//! ```rust
//! let stream = TcpStream::connect("192.168.0.12:9100").await.unwrap();
//! let mut pos = epson::AsyncWriter::open(Model::T20II, Box::new(stream)).await.unwrap();
//!
//! pos.speed(5).await.unwrap();
//! pos.write_all(b"HACK THE PLANET\n").await.unwrap();
//! pos.feed(5).await.unwrap();
//! pos.cut().await.unwrap();
//! ```

mod barcode;
mod commands;
mod epson_image;
mod models;
mod write;

#[cfg(feature = "tokio")]
mod async_tokio;

pub use barcode::Barcode;
pub use commands::{Alignment, CharacterSet, Command, HriPosition};
use epson_image::ImageBuffer;
pub use models::Model;
pub use write::Writer;

#[cfg(feature = "tokio")]
pub use async_tokio::{AsyncWriter, Error as AsyncWriterError};

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

    /// This is returned when a barcode has an invalid number of digits.
    InvalidBarcodeLength,

    /// This is returned when a barcode contains invalid characters.
    InvalidBarcodeCharacters,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

// vim: foldmethod=marker
