#![no_main]
#![no_std]

mod serial_setup;

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use microbit::{
    hal::{
        prelude::*,
        uarte::{self, Baudrate, Parity},
    },
    Board,
};
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Microbit send string");

    let board = if let Some(board) = Board::take() {
        board
    } else {
        panic!("Failed to take the Microbit board.");
    };

    // UartePort is a struct that wraps a Uarte instance and implements the
    // Write trait. The UartePort struct is used to send data over the serial
    // port. The UartePort struct has a new() associated function that returns
    // a UartePort instance.

    // code block returns a UartePort instance and assigns it to serial
    let mut serial = {
        // UartePort::new() is an associated function that returns a UartePort
        let serial = uarte::Uarte::new(
            // board.UARTE0 - UARTE0 is the peripheral that is used for
            // the serial communication
            board.UARTE0,
            // board.uart - UART pins are used for the serial communication
            board.uart.into(),
            // Parity::EXCLUDED - No parity bit is used
            // A parity bit is an error-checking bit that is added to the
            // data bits to ensure that the number of 1s in the data bits
            // is even or odd
            Parity::EXCLUDED,
            // Baudrate::BAUD115200 - The baud rate is set to 115200 bits
            // per second
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // Option 1: Write individual characters to the serial port
    // for chr in "The quick brown fox jumps over the lazy dog.".chars() {
    //     // nb::block! - blocks until the serial port is ready to write
    //     if nb::block!(serial.write(chr as u8)).is_err() {
    //         panic!("Error writing to the serial port");
    //     }
    // }

    // Option 2: Write a string to the serial port with the write! macro
    // write! - writes a formatted string to the serial port
    if write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").is_err() {
        panic!("Error writing to the serial port");
    }
    
    if nb::block!(serial.flush()).is_err() {
        panic!("Error flushing the serial port");
    }

    loop {}
}
