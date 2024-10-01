#![no_main]
#![no_std]

extern crate alloc;

use embedded_alloc::LlffHeap as Heap;

// alloc::boxed::Box is a smart pointer with the type signature Box<T> that
// provides heap allocation for values of type T. When a value goes out of
// scope, its destructor is called, the memory is deallocated, and the value
// is dropped.
// alloc is usually not used directly, but through higher-level abstractions,
// but because we are using a custom heap allocator, we need to use it directly.
use alloc::boxed::Box;

use cortex_m_rt::entry;
use microbit as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// static - The variable is allocated at compile time and is stored in the
// read-only memory of the program.
// global_allocator - The global_allocator attribute is used to specify the
// allocator to use for global memory allocation. Box uses the global allocator
// to allocate memory on the heap.
#[global_allocator]
static HEAP: Heap = Heap::empty(); // Create an empty heap

#[entry]
fn main() -> ! {

    // Initialize the allocator BEFORE you use it
    {
        // The MaybeUninit type is a type that represents an uninitialized value.
        use core::mem::MaybeUninit;

        // The size of the heap in bytes
        const HEAP_SIZE: usize = 1024;
        // The heap memory allocated at compile time as an array accessed through
        // static mutable variable which make the array accessible from anywhere
        // and mutable.
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        // unsafe turns off the borrow checker
        unsafe {
            // warning: shared reference to mutable static
            // The error "shared reference to mutable static" occurs
            // because Rust's ownership model enforces strict rules
            // to ensure memory safety, particularly around mutable and
            // shared references. In Rust, you cannot have a shared
            // reference (&T) to a mutable static variable (static mut T)
            // because it can lead to data races and undefined behavior.

            // the allocated heap memory is used to initialize the heap
            HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE)
        }
    }    

    rtt_init_print!();

    // now Box can be used
    let boxed_num = Box::new(42);

    // dereference the Box to get the value
    rprintln!("The meaning of life is: {}", *boxed_num);

    #[allow(clippy::empty_loop)]
    loop { }
}
