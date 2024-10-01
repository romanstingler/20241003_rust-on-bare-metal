# Improved LED Project

1. Create a new bare metal LED Rust project using these instructions: [LED Bare Metal Project](./03_PROGRAM_LED_PROJECT.md). Name the project `led_microbit_improved`.

    Note: If you created the `led_microbit` project, and you made no changes to it, please feel free to copy the project to a new directory and rename it to `led_microbit_improved`. You will need to update the `Cargo.toml` file to reflect the new project name, and add the project to the workspace `Cargo.toml` file.

1. Open the `src/main.rs` file in the `led_microbit_improved` project.

1. Review the following line of code in the `main` function.

    ```rust
    let board = Board::take().unwrap();
    ```

    What is wrong with this line of code? What if the `take()` associated function returns the `None` variant? Is this situation being properly handled with the call to `unwrap()`?

1. Before we fix this, what does it look like to panic within an embedded program, and how can we see this panic message?

    1. Add the following code above the `board` variable declaration.

        ```rust
        panic!("Failed to take the Microbit board.");
        ```

        Because the project includes the `panic-halt` crate, the program will halt when a panic occurs. The panic message will be displayed on the RTT console.

1. Build and flash the project to the Microbit.

    ```sh
    cargo embed --target thumbv7em-none-eabihf
    ```

    The program should flash, but the LEDs will not light up. The panic message will be displayed on the RTT console.

1. Now that we can panic and view the panic message, let's upgrade the code to handle the `None` variant returned by the `take()` function.

    - Remove the previous `panic!` statement.

    - Replace the line of code that declares and assigns the `board` variable with the following code.

    ```rust
    let board = if let Some(board) = Board::take() {
        board
    } else {
        panic!("Failed to take the Microbit board.");
    };
    ```

    Using the `if-let` expression, we can check if the `Board::take()` function returns `Some(board)`. If it does, we assign the board to the `board` variable. If it returns `None`, we panic with the message "Failed to take the Microbit board."

1. Build and flash the project to the Microbit.

    ```sh
    cargo embed
    ```

    The program should flash, and the LEDs will light up. Ideally, no panic message will be displayed on the RTT console; instead, the "Hello from micro:bit!" message will be displayed repeatedly.
