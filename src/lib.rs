#[macro_use]
extern crate log;
extern crate serial;

use std::io;
use std::io::{Write};
use std::sync::{Mutex, Arc};
use std::time::Duration;

use serial::prelude::*;

pub mod prelude {
    pub use ::{Printer, Print, Command, Justification};
}

#[derive(Debug)]
pub enum Command {
    ESC = 27,
    DC2 = 18,
    LF = 10,
    DASH = 45,
    EXCL = 69,
    DOTS0 = 0xB0,
    DOTS1 = 0xB1,
    DOTS2 = 0xB2,
    SPACE = 0x20
}

#[derive(Debug)]
pub enum Justification {
    Left = 0,
    Center = 1,
    Right = 2
}

#[derive(Clone)]
pub struct Printer {
    port: Arc<Mutex<Box<SerialPort + Send>>>
}

impl Printer {
    pub fn new(dev_path: &str) -> io::Result<Printer> {
        let port = Printer::open_serial_port(dev_path)?;

        let mut printer = Printer{port: Arc::new(Mutex::new(port))};

        trace!("Configuring printer");
        printer.configure(7, 120, 4)?;

        trace!("Feeding line on start");
        printer.feed()?;

        return Ok(printer);
    }

    fn open_serial_port(dev_path: &str) -> io::Result<Box<SerialPort + Send>> {
        trace!("Opening serial port at {}", dev_path);

        let mut port = serial::open(dev_path)?;
        trace!("Configuring serial port");
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud19200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        port.set_timeout(Duration::from_millis(1000))?;

        Ok(Box::new(port))
    }

    pub fn configure(&mut self, dots: u8, time: u8, interval: u8) -> io::Result<()> {
        // LF 7
        let cmd = [Command::ESC as u8, 55u8, dots, time, interval];
        self.write(&cmd)?;
        self.flush()
    }

    pub fn print<T: Print>(&mut self, item: T) -> io::Result<()> {
        item.print(self)
    }

    pub fn run_test(&mut self) -> io::Result<()> {
        trace!("Triggering test page");
        self.justify(Justification::Center)?;

        let out = [Command::DOTS2 as u8; 30];
        self.write(&out)?;
        self.feed()?;
        self.feed()
    }

    pub fn justify(&mut self, just: Justification) -> io::Result<()> {
        trace!("Setting justification {:?}", just);
        let cmd = [Command::ESC as u8, 97u8, just as u8];

        self.write(&cmd)?;
        self.flush()
    }

    pub fn feed(&mut self) -> io::Result<()> {
        self.feed_n(1)
    }

    pub fn feed_n(&mut self, n: u8) -> io::Result<()> {
        for _ in 0..n {
            self.write(&[Command::LF as u8])?;
        }
        self.flush()
    }

    pub fn set_underline(&mut self, n: u8) -> io::Result<()> {
        let cmd = [Command::ESC as u8, Command::DASH as u8, n];
        self.write(&cmd)?;
        self.flush()
    }

    pub fn set_bold(&mut self, flag: bool) -> io::Result<()> {
        let cmd = [Command::ESC as u8, Command::EXCL as u8, flag as u8];
        self.write(&cmd)?;
        self.flush()
    }
}

impl Write for Printer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let port_lock = self.port.clone();
        let mut port = port_lock.lock().unwrap();

        port.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let port_lock = self.port.clone();
        let mut port = port_lock.lock().unwrap();

        port.flush()
    }
}

pub trait Print {
    fn print(&self, printer: &mut Printer) -> io::Result<()>;
}
