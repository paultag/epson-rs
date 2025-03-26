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

/// Supported barcode types
#[derive(Debug, Clone)]
pub enum Barcode {
    /// UPC-A barcode format. Requires exactly 11-12 numeric digits.
    UPCA(Vec<u8>),
}

impl Barcode {
    /// Create a new UPC-A barcode.
    ///
    /// UPC-A requires exactly 11 or 12 numeric digits (0-9).
    /// If 11 digits are provided, the check digit will be calculated automatically.
    /// If 12 digits are provided, the last digit is assumed to be the check digit.
    pub fn new_upca(data: &[u8]) -> Result<Self, Error> {
        // Check length - UPC-A requires 11 or 12 digits
        if data.len() != 11 && data.len() != 12 {
            return Err(Error::InvalidBarcodeLength);
        }

        // Check that all characters are digits
        if !data.iter().all(|&b| b.is_ascii_digit()) {
            return Err(Error::InvalidBarcodeCharacters);
        }

        Ok(Barcode::UPCA(data.to_vec()))
    }

    /// Returns the raw data for the barcode.
    pub(crate) fn data(&self) -> &[u8] {
        match self {
            Barcode::UPCA(data) => data,
        }
    }

    /// Returns the Epson-specific barcode type code.
    pub(crate) fn barcode_type(&self) -> u8 {
        match self {
            Barcode::UPCA(_) => 65, // UPC-A type code as specified
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upca_valid() {
        // Valid 11-digit UPC-A
        let barcode = Barcode::new_upca(b"12345678901").unwrap();
        assert_eq!(barcode.barcode_type(), 65);
        assert_eq!(barcode.data(), b"12345678901");

        // Valid 12-digit UPC-A
        let barcode = Barcode::new_upca(b"123456789012").unwrap();
        assert_eq!(barcode.barcode_type(), 65);
        assert_eq!(barcode.data(), b"123456789012");
    }

    #[test]
    fn test_upca_invalid_length() {
        // Too short
        assert!(matches!(
            Barcode::new_upca(b"1234567890"),
            Err(Error::InvalidBarcodeLength)
        ));

        // Too long
        assert!(matches!(
            Barcode::new_upca(b"1234567890123"),
            Err(Error::InvalidBarcodeLength)
        ));
    }

    #[test]
    fn test_upca_invalid_characters() {
        // Non-numeric characters
        assert!(matches!(
            Barcode::new_upca(b"1234567890A"),
            Err(Error::InvalidBarcodeCharacters)
        ));
    }
}

// vim: foldmethod=marker
