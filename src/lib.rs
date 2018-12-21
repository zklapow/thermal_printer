#![no_std]

use embedded_hal::serial;
use embedded_hal::blocking::serial::write::Default;
use embedded_hal::prelude::*;
use nb::{Result};

pub mod prelude {
    pub use super::{ThermalPrinter, Command, Justification};
    pub use embedded_hal::serial::Write as _thermal_printer_serial_Write;
}

#[derive(Debug)]
pub enum Command {
    ESC = 27,
    DC2 = 18,
    LF = 10,
    DASH = 45,
    EXCL = 69,
    SPACE = 0x20
}

#[derive(Debug)]
pub enum Justification {
    Left = 0,
    Center = 1,
    Right = 2
}

#[derive(Clone)]
pub struct ThermalPrinter<T> where T: serial::Write<u8> {
    port: T,

    dot_print_time: u64,
    dot_feed_time: u64,
}

impl <T> ThermalPrinter<T> where T: serial::Write<u8> {
    pub fn new(port: T) -> ThermalPrinter<T> {
        // The default times here are copied from adafruit's thermal printer arduino library
        return ThermalPrinter {
            port,
            dot_print_time: 30000u64,
            dot_feed_time: 2100u64,
        }
    }

    pub fn configure(&mut self, dots: u8, time: u8, interval: u8) -> Result<(), T::Error> {
        // LF 7
        let cmd = [Command::ESC as u8, 55u8, dots, time, interval];
        self.write_all(&cmd)?;
        self.flush()
    }

    pub fn run_test(&mut self) -> Result<(), T::Error> {
        self.justify(Justification::Center)?;

        let out = [Command::DC2 as u8, 'T' as u8];
        self.write_all(&out)?;
        self.flush()
    }

    pub fn justify(&mut self, just: Justification) -> Result<(), T::Error> {
        let cmd = [Command::ESC as u8, 97u8, just as u8];

        self.write_all(&cmd)?;
        self.flush()
    }

    pub fn feed(&mut self) -> Result<(), T::Error> {
        self.feed_n(1)
    }

    pub fn feed_n(&mut self, n: u8) -> Result<(), T::Error> {
        let cmd = [Command::ESC as u8, 'd' as u8, n];

        self.write_all(&cmd)?;
        self.flush()
    }

    pub fn set_underline(&mut self, n: u8) -> Result<(), T::Error> {
        let cmd = [Command::ESC as u8, Command::DASH as u8, n];
        self.write_all(&cmd)?;
        self.flush()
    }

    pub fn set_bold(&mut self, flag: bool) -> Result<(), T::Error> {
        let cmd = [Command::ESC as u8, Command::EXCL as u8, flag as u8];
        self.write_all(&cmd)?;
        self.flush()
    }

    /// Consume the printer, freeing the underlying serial port
    pub fn free(self) -> T {
        self.port
    }

    fn write_all(&mut self, buffer: &[u8]) -> Result<(), T::Error> {
        self.bwrite_all(buffer).map_err(|e| nb::Error::Other(e))
    }
}

impl <T> serial::Write<u8> for ThermalPrinter<T> where T: serial::Write<u8> {
    type Error = T::Error;

    fn write(&mut self, word: u8) -> Result<(), Self::Error> {
        self.port.write(word)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.port.flush()
    }
}

impl <T> Default<u8> for ThermalPrinter<T> where T: serial::Write<u8> {}
