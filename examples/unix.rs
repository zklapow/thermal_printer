extern crate thermal_printer;
extern crate serial;
extern crate serial_embedded_hal;

use serial::{Baud19200, Bits8, ParityNone, Stop1, FlowNone};
use serial_embedded_hal::{Serial, PortSettings};

use thermal_printer::prelude::*;

use std::thread::sleep;

fn main() {
    println!("Opening serial port...");

    let port_settings = PortSettings {
        baud_rate: Baud19200,
        char_size: Bits8,
        parity: ParityNone,
        stop_bits: Stop1,
        flow_control: FlowNone,
    };

    println!("Serial port open");

    let port = Serial::new("/dev/tty.usbserial-A700eX73", &port_settings)
        .expect("Failed to open serial port");

    let (tx_port, _) = port.split();

    let mut printer = ThermalPrinter::new(tx_port);

    printer.configure(11, 120, 40);

    println!("Feeding 3 lines");
    printer.feed_n(3).expect("Feed lines failed");
    println!("Running self test");
    printer.run_test().expect("Self test failed");

    sleep(std::time::Duration::from_secs(1));

    println!("Done!")
}