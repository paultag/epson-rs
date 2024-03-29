# epson: rust bindings to the Epson thermal printer line encoding scheme

[epson-rs](https://github.com/paultag/epson-rs) are Rust bindings to the
Epson Point of Sale (POS) thermal printers' printer format.

Currently, this library supports a limited number of commands, and some
basic interfaces for both synchronous Rust as well as async
Rust through [tokio](https://tokio.rs/), behind the `tokio` feature.

Docs can be found on [docs.rs](https://docs.rs/epson/latest/epson/),
and information about the latest release can be found on
[crates.io](https://crates.io/crates/epson).

## Example Programs

Check the `examples` directory for some program that use the `epson`
library to print things to a printer.
