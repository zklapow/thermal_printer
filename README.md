[![Build Status](https://travis-ci.org/zklapow/thermal_printer.svg?branch=master)](https://travis-ci.org/zklapow/thermal_printer)

This library provides a simple high level API for interacting with thermal printers like those you can get from [Sparkfun](https://www.sparkfun.com/products/14970) or [Adafruit](https://www.adafruit.com/product/597). It is written to be `no-std` compatible but simply needs a serial port and so will work equally well on linux or other operating systems via the [`serial`](https://crates.io/crates/serial) crate.

## WIP

Many pieces of functionality are not yet supported. In the future this crate aims to support the full range of functions these printers provide including:

- Loading and printing bitmap images
- Bar codes
- Custom characters
- Multiple character sets
- Reading printer status

And many more! PR's are welcome.

## Examples

See the [examples folder](./examples).

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](./LICENSE))

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
