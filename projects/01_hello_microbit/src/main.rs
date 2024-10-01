#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};


#[entry]
fn main() -> ! {
    // RTT (Real-Time Transfer) initialization
    // This macro initializes RTT printing. It must be called once at the
    // beginning of the program.

    // RTT stands for Real-Time Transfer. It is a technology developed by
    // SEGGER that allows for bi-directional communication between a target
    // microcontroller and a host computer during debugging. RTT is
    // particularly useful for logging and debugging in real-time without
    // significantly impacting the performance of the target application.

    rtt_init_print!();

    loop {
        // RTT printing
        rprintln!("Hello from micro:bit!");
    }
}
