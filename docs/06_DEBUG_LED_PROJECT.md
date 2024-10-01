# Debug LED Project

1. Create a new bare metal Improved LED Rust project using these instructions: [LED Bare Metal Project](./04_IMPROVED_LED_PROJECT.md). Name the project `debug_led_microbit`.

    Note: If you created the `led_microbit_improved` project, and you made no changes to it, please feel free to copy the project to a new directory and rename it to `debug_led_microbit`. You will need to update the `Cargo.toml` file to reflect the new project name, and add the project to the workspace `Cargo.toml` file.

1. Open the `Embed.toml` file in the `debug_led_microbit` project.

    Following the chip configuration.

    ```toml
    [default.general]
    chip = "nrf52833_xxAA"

    [default.rtt]
    enabled = true
    ```

    Replace the original configuration with the following configuration.

    ```toml
    [default.general]
    chip = "nrf52833_xxAA"

    [default.rtt]
    enabled = false    

    [default.reset]
    halt_afterwards = true

    [default.gdb]
    enabled = true
    ```

    - The `halt_afterwards` configuration will halt the program after flashing.
    - The `gdb` configuration will enable the GDB stub.

1. Build and flash the project to the micro:bit.

    ```sh
    cargo embed
    ```

    After the flash, it should wait for a GDB connection. You should see something similar to this in the console.

    ```text
    Finished in 2.526s
    GDB stub listening at 127.0.0.1:1337
    ```

    Observe that the LEDs are not flashing. This is because the program is halted waiting for a GDB connection.

1. Next, we will use GDB to debug the program on the microbit. Use the appropriate `gdb` executable for your system. Here is a list of possible executables. Refer to the original setup instructions to confirm the executable you installed for your system.

    - `gdb` (Linux)
    - `arm-none-eabi-gdb` (macOS)
    - `arm-none-eabi-gdb.exe` (Windows)

    For example, on macOS, from the `module_11_bare_metal` folder, run the following command in a new terminal window.

    ```sh
    arm-none-eabi-gdb ../target/thumbv7em-none-eabihf/debug/debug_led_microbit
    ```

    If prompted, `c` to continue without paging.

    Note: The message --Type <RET> for more, q to quit, c to continue without paging-- is a prompt from a pager program, such as less or more, which is used to view long text outputs in the terminal. Here's what each option means:

    - <RET>: Pressing the Return (Enter) key will show the next page of text.
    - q: Pressing q will quit the pager and return to the command prompt.
    - c: Pressing c will continue displaying the rest of the text without pausing for paging.

    In this case, typing c will display the remaining text continuously without stopping for further prompts.

1. At the GDB prompt, connect to the micro:bit using the following command. Use the URL from the GDB stub message.

    ```sh
    target remote :1337
    ```

1. Next, we will set a breakpoint at the `previous_led = *current_led;` statement. Review your `src/main.rs` file to see the line number of the line. Set the break point using the following command with your line number.

    ```sh
    break main.rs:60
    ```

1. Next, run the `continue` command to run until the breakpoint is hit. This command may not work on all operating systems.

    ```sh
    continue
    ```

1. With the following GDB command, View the source code where the breakpoint is.

    ```sh
    layout src
    ```

1. Print the value of the `leds` variable.

    ```sh
    print leds
    ```

    Notice the first element of the array should be 1. If the first element is not 1, notice that element that is 1, and the next element should be 1 after the next two steps.

1. Next, run the `continue` command to run until the breakpoint is hit.

    ```sh
    continue
    ```

1. Print the value of the `leds` variable.

    ```sh
    print leds
    ```

    Notice the second element (or next element) of the array should be 1, and prior element is now 0.

1. List the breakpoints.

    ```sh
    info breakpoints
    ```

1. Delete the breakpoint using the breakpoint number.

    ```sh
    delete 1
    ```

1. Continue running the program.

    ```sh
    continue
    ```

1. To exit GDB, type `<CTRL-C>`. Then type `quit`, and type `y` to confirm. Close the terminal window.

1. Stop the flash debugging session in the first terminal window by typing `<CTRL-C>`. Close the terminal window.

1. As great as command-line GDB is, we can debug the program using Visual Studio Code.

1. Install the `probe-rs.probe-rs-debugger` extension in Visual Studio Code.

1. On the main menu, click `Run`, then `Add Configuration...`.

1. Copy the following configuration into `configurations` array of the `launch.json` file.

    ```json
    {
      "type": "probe-rs-debug",
      "request": "launch",
      "name": "Debug application",
      "cwd": "${workspaceFolder}",
      "runtimeExecutable": "probe-rs",
      "runtimeArgs": [
        "dap-server"
      ],
      "chip": "nRF52833_xxAA",
      "flashingConfig": {
        "flashingEnabled": true,
        "haltAfterReset": false,
        "formatOptions": {}
      },
      "coreConfigs": [
        {
          "coreIndex": 0,
          "programBinary": "target/thumbv7em-none-eabihf/debug/debug_led_microbit",
        }
      ],
      "env": {
        "RUST_LOG": "info"
      },
      "consoleLogLevel": "Console"
    }
    ```

1. Open the Debug view in Visual Studio Code, by clicking the triangle with bug icon on the Activity Bar on the left-side of the Visual Studio Code window.

1. At the top of the Debug view, click the green play button to start the debugger. This will build, flash and start the debugger. A debug toolbar will appear at the top of the window. Do not click any of the buttons.

1. Open the `src/main.rs` file in the editor. Set a breakpoint on the following line of code.

    ```rust
    previous_led = *current_led;
    ```

1. You can review the the variables, and step through the code using the debug toolbar.

1. Explore setting a couple of breakpoints, and stepping through the code. When done click the red square to stop the debugger.
