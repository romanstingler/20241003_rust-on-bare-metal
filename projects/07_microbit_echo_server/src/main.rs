#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{prelude::*, uarte::{self, Baudrate, Parity}},
    Board,
};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = if let Some(board) = Board::take() {
        board
    } else {
        panic!("Failed to take the Microbit board.");
    };

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    loop {

        // serial.read() - reads a byte from the serial port
        match nb::block!(serial.read()) {
            Ok(byte) => {
                rprintln!("Received: {}", byte as char);
                if nb::block!(serial.write(byte)).is_err() {
                    panic!("Failed to write byte");
                }
                if nb::block!(serial.flush()).is_err() {
                    panic!("Failed to flush serial port");
                }
            }
            Err(_) => {
                panic!("Failed to read byte");
            }
        }

    }
}
