#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::gpiote::Gpiote,
    pac::{self, interrupt},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// static - a static variable that is safe to share between threads
// Mutex - a mutual exclusion primitive useful for sharing memory safely
// RefCell - a mutable memory location with dynamically checked borrow rules
static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

// A critical section in programming is a segment of code that must be executed
// by only one thread at a time to prevent race conditions. In Rust, critical
// sections are often used in embedded systems to manage access to shared
// resources and ensure data consistency.
//
// 1. Disable Interrupts: When entering a critical section, interrupts are
//    typically disabled to prevent other threads or interrupt service routines
//    (ISRs) from preempting the current thread. This ensures that the critical
//    section code runs to completion without interruption.
//
// 2. Execute Critical Code: The code within the critical section is executed.
//    Since interrupts are disabled, this code can safely access and modify
//    shared resources without the risk of race conditions.
//
// 3. Enable Interrupts: After the critical section code has executed, interrupts
//    are re-enabled. This allows the system to handle any pending interrupts that
//   occurred while the critical section was being executed.
//
// In Rust, this can be achieved using specific constructs provided by the embedded
// systems libraries, such as the cortex-m crate for ARM Cortex-M processors.

// interrupt handler - runs when the GPIOTE interrupt is triggered
#[interrupt]
fn GPIOTE() {
    // free - executes a closure with interrupts disabled so that the
    // GPIO resource can be accessed
    // cs - critical section token that is used to enable interrupts
    free(|cs| {
        // get a reference to the GPIO resource through run-time borrowing
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            // check if the event for channel 0 has been triggered
            let a_pressed = gpiote.channel0().is_event_triggered();

            if a_pressed {
                rprintln!("Button A pressed!");
            }

            // reset the event for channel 0 to allow it to trigger again
            gpiote.channel0().reset_events();
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // take - attempts to take ownership of the Microbit board, returning
    // Some(board) if successful or None if the board is already taken
    let board = if let Some(board) = Board::take() {
        board
    } else {
        panic!("Failed to take the Microbit board.");
    };

    // create a new GPIOTE instance using the GPIOTE peripheral from
    // the board
    let gpiote = Gpiote::new(board.GPIOTE);

    // configure channel 0 of the GPIOTE instance to monitor the input
    let channel0 = gpiote.channel0();

    // pin connected to button A on the Microbit board
    // hi_to_lo - configure the channel to trigger an event on a high-to-low
    // transition of the input signal
    // enable_interrupt - enable the interrupt for the channel
    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();

    //  reset the event for channel 0 to ensure it is ready
    channel0.reset_events();

    free(move |cs| {
        // store the GPIO resource in the shared Mutex for access in the
        // interrupt handler
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe {
            // unmask the GPIOTE interrupt so the system can respond to it
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }

        // clear any pending GPIOTE interrupts
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    });

    loop {}
}
