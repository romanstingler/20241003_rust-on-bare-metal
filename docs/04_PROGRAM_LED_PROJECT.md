# Program LED Project

1. Create a new bare metal Rust project using these instructions: [Create a New Bare Metal Project](./02_CREATE_BARE_METAL_PROJECT.md).

    Note: If you created the `hello_microbit` project, and you made no changes to it, please feel free to copy the project to a new directory and rename it to `led_microbit`. You will need to update the `Cargo.toml` file to reflect the new project name, and add the project to the workspace `Cargo.toml` file.

1. Open the `src/main.rs` file in the `led_microbit` project.

1. Update the `use microbit as _;` with the following `use` statement.

    - The `board::Board` struct provides access to the Microbit's peripherals.
    - The `display::blocking::Display` trait provides the `set_pixel` method to set the LED display's pixels.
    - The `hal::Timer` struct provides access to the Microbit's timer peripheral.

    ```rust
    use microbit::{
        board::Board,
        display::blocking::Display,
        hal::Timer,
    };
    ```

1. Below the `use` statements section, add the following variable declaration.

    The `PIXELS` array contains the tuple coordinates of 16 pixels on the Microbit's LED display that will be toggled on and off. These pixels are the outline of the 25x25 pixel square.

    ```rust
    const PIXELS: [(usize, usize); 16] = [
        (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
        (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
    ];
    ```

1. Following the `rtt_init_print!();` statement, add the following code.

    ```rust
    let board = Board::take().unwrap();
    ```

    The `Board::take()` method returns the `Board` struct, which provides access to the Microbit's peripherals. The `unwrap()` method is used to extract the `Board` struct from the `Option` type returned by the `Board::take()` method.

    The `Board::take()` method will return an instance of the board the first time it is called. It will return only None on subsequent calls.

1. Below the `board` variable declaration and assignment, add the following code.

    ```rust
    let mut timer = Timer::new(board.TIMER0);
    ```

    The `Timer::new()` method creates a new `Timer` struct that provides access to the Microbit's timer peripheral. The `board.TIMER0` field is the timer peripheral. The variable is mutable because the timer will be used to create delays.

1. Below the `timer` variable declaration and assignment, add the following code.

    ```rust
    let mut display = Display::new(board.display_pins);
    ```

    The `Display::new()` method creates a new `Display` struct that provides access to the Microbit's LED display. The `board.display_pins` field is the LED display pins. The variable is mutable because the display will be used to set the LED display's pixels.

1. Below the `display` variable declaration and assignment, add the following code.

    ```rust
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    ```

    The `leds` array contains the state of the 5x5 LED display. Each element in the array is an array of 5 elements that represent the state of the LED display's row. The initial state of the LED display is all pixels turned off.

1. Below the `leds` variable declaration and assignment, add the following code.

    ```rust
    let mut previous_led = (0,0);
    ```

    This will track the previous LED that was turned on, so it can be turned off before turning on the next LED.

1. In the `loop`, below the `rprintln!("Hello from micro:bit!");` statement, add the following `for-in` loop.

    ```rust
    // iterate over the PIXELS array
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
    ```

    Save the file.

1. Build and flash the program.

  ```sh
  cargo embed
  ```
