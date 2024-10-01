#![no_main]
#![no_std]

mod serial_setup;

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
use serial_setup::UartePort;

// The core principle behind heapless is that its data structures are
// backed by a static memory allocation. For example, you can think of
// heapless::Vec as an alternative version of std::Vec with fixed capacity
// and that can't be re-allocated on the fly (e.g. via push).

// All heapless data structures store their memory allocation inline and
// specify their capacity via their type parameter N. This means that you
// can instantiate a heapless data structure on the stack, in a static
// variable, or even in the heap.

use heapless::Vec;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = if let Some(board) = microbit::Board::take() {
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

    // The heapless::Vec type is a fixed sized array that can grow up to
    // a maximum capacity of N. The Vec type is generic over the element
    // type T and the maximum capacity N. The Vec type is a wrapper around
    // an array of type T with a length field that keeps track of the number
    // of elements currently in the array.
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            match nb::block!(serial.read()) {
                Ok(byte) => {
                    if byte == b'\r' {
                        break;
                    }

                    // Because the buffer is a heapless::Vec, we can push bytes
                    // into it. If the fixed-sized buffer is full, we will print
                    // an error message and break out of the loop.
                    if buffer.push(byte).is_err() {
                        if write!(serial, "Buffer full. Please try again.\r\n").is_err() {
                            panic!("Error writing to serial port");
                        }
                        if nb::block!(serial.flush()).is_err() {
                            panic!("Error flushing serial port");
                        }
                        break;
                    }                    
                },
                Err(_) => {
                    panic!("Error reading from serial port");
                }
            }

        }

        for byte in buffer.iter().rev() {
            if nb::block!(serial.write(*byte)).is_err() {
                panic!("Error writing to serial port");
            }
        }
        if nb::block!(serial.flush()).is_err() {
            panic!("Error flushing serial port");
        }
    }
}
