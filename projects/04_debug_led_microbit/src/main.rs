#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

const PIXELS: [(usize, usize); 16] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = if let Some(board) = Board::take() {
        board
    } else {
        panic!("Failed to take the Microbit board.");
    };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut previous_led = (0, 0);

    loop {
        rprintln!("Hello from micro:bit!");
        for current_led in PIXELS.iter() {
            // turn off the previous LED
            leds[previous_led.0][previous_led.1] = 0;
            // turn on the current LED
            leds[current_led.0][current_led.1] = 1;
            // update the display
            display.show(&mut timer, leds, 30);
            // copy the values from the current LED to the previous LED
            previous_led = *current_led;
        }
    }
}
