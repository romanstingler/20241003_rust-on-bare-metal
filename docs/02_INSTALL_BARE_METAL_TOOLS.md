# Install Bare Metal Project Tools

This document provides instructions for installing the tools needed to work with the Microbit in a bare metal environment on Linux, macOS, and Windows. There are three parts to the installation:

1. Install the Rust toolchain.
1. Install and configure operating system specific tools.
1. Verify the Microbit connection.

## Install Project Tools

1. Install the `llvm-tools` to convert the compiled Rust code into a binary file that we can load onto the Microbit.

    ```sh
    rustup component add llvm-tools
    ```

    The `component` command manages optional parts of the Rust toolchain. The `add` subcommand installs a component. The `llvm-tools` component provides the `llvm-objcopy` tool, which we will use to convert the compiled Rust code into a binary file that we can load onto the Microbit.

1. Install the `cargo-binutils` crate to provide the cargo integrated command wrapper for `llvm-tools`.

    ```sh
    cargo install --locked cargo-binutils --vers 0.3.3
    ```

    The cargo `install` command is used to install Rust binaries from crates. It downloads the specified crate from crates.io, compiles it, and installs the resulting binary to the local cargo bin directory (usually ~/.cargo/bin).

    The `cargo-binutils` crate provides the `cargo objcopy` command, which we will use to convert the compiled Rust code into a binary file that we can load onto the Microbit.

    The `cargo objcopy` command is a wrapper around the `llvm-objcopy` tool provided by the `llvm-tools` component. It simplifies the process of converting the compiled Rust code into a binary file that we can load onto the Microbit.

1. Install the `probe-rs-tools` crate to provide tools for working with the Microbit.

    ```sh
    cargo install --locked probe-rs-tools --vers '^0.24'
    ```

    The `probe-rs-tools` crate is an embedded debugging and target interaction toolkit. It enables its user to program and debug microcontrollers via a debug probe. It helps withs:

    - Flashing firmware to ARM and RISC-V targets. More architectures are in the works.
    - Reading and writing memory, running, halting, setting and reading breakpoints and resetting the target via SWD and JTAG.
    - Running firmware on the target and getting back logs via RTT and defmt and printing a stacktrace on panic.
    - Debugging the target via VS Code with running RTT logs, memory inspection and more.

1. Verify the installation of `cargo embed`.

    ```sh
    cargo embed --version
    ```

    The `cargo embed` command is a wrapper around the `probe-rs` tool provided by the `probe-rs` crate. It simplifies the process of loading the compiled Rust code onto the Microbit.

## Operating System Specific Instructions

The following links provide instructions for installing the tools on different operating systems:

- [Linux (Debian/Ubuntu)](#linux-debianubuntu)
- [macOS](#macos)
- [Windows](#windows)

### Linux (Debian/Ubuntu)

1. Install `gdb-multiarch` and `minicom` to debug the Microbit.

    ```sh
    sudo apt-get install gdb-multiarch minicom
    ```

    The `gdb-multiarch` package provides the `gdb-multiarch` command, which is a version of the GNU Debugger that supports multiple architectures. We will use it to debug the Microbit.

    The `minicom` package provides the `minicom` command, which is a terminal emulator that we will use to communicate with the Microbit.

1. Create/Edit the file `/etc/udev/rules.d/69-microbit.rules`. Add the following content to it. Save the file.

    ```text
    # CMSIS-DAP for microbit

    ACTION!="add|change", GOTO="microbit_rules_end"

    SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", TAG+="uaccess"

    LABEL="microbit_rules_end"
    ```

    The rules file tells the system to give the user access to the Microbit when it is plugged in. The `ATTR{idVendor}` and `ATTR{idProduct}` values are the USB vendor and product IDs for the Microbit. The `TAG+="uaccess"` directive tells the system to give the user access to the Microbit.

1. Open a terminal, and run the following command.

    ```sh
    sudo udevadm control --reload
    ```

    The `udevadm` command is used to control the udev daemon, which manages device nodes in the `/dev` directory. The `control --reload` subcommand tells the udev daemon to reload its rules.

1. If any boards are plugged in, unplug and plug them back in. Alternatively, run the following command in the terminal.

    ```sh
    sudo udevadm trigger
    ```

    The `udevadm` command is used to control the udev daemon, which manages device nodes in the `/dev` directory. The `trigger` subcommand tells the udev daemon to reapply its rules.

1. Connect the Microbit to the computer using a USB cable.

1. In the terminal, run the following command.

    ```sh
    lsusb | grep -i "NXP ARM mbed"
    ```

    The output will look something like this:

    ```text
    Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
    ```

    The `lsusb` command lists USB devices connected to the system. The `grep -i "NXP ARM mbed"` command filters the output to show only lines that contain the string "NXP ARM mbed". If the Microbit is connected, you should see a line that contains the string "NXP ARM mbed".

1. In the terminal, run the following command. Replace `001` and `065` with the bus and device numbers from the previous step.

    ```sh
    ls -l /dev/bus/usb/001/065
    ```

    Verify the permissiond are set correctly. The output will look something like this:

    ```text
    crw-rw-r--+ 1 nobody nobody 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
    ```

    The permissions should be crw-rw-r--+, note the + at the end, then see your access rights by running the following command.

1. Run the following command in the terminal to verify your access rights. Replace `001` and `065` with the bus and device numbers from the previous step.

    ```sh
    getfacl /dev/bus/usb/001/065
    ```

    You should see your username in the list above with the rw- permissions, if not, then check your udev rules and try re-loading them with:

    ```sh
    sudo udevadm control --reload
    sudo udevadm trigger
    ```

### macOS

1. Install

    ```sh
    brew install arm-none-eabi-gdb
    ```

    The `arm-none-eabi-gdb` package provides the `arm-none-eabi-gdb` command, which is a version of the GNU Debugger that supports the ARM architecture. We will use it to debug the Microbit.

1. Install `minicom` to communicate with the Microbit.

    ```sh
    brew install minicom
    ```

    The `minicom` package provides the `minicom` command, which is a terminal emulator that we will use to communicate with the Microbit.

1. Install `lsusb` to list USB devices.

    ```sh
    brew install lsusb
    ```

    The `lsusb` package provides the `lsusb` command, which lists USB devices connected to the system.

### Windows

1. Install `arm-none-eabi-gcc` for Windows from [https://developer.arm.com/downloads/-/gnu-rm](https://developer.arm.com/downloads/-/gnu-rm)

    The `arm-none-eabi-gcc` package provides the `arm-none-eabi-gcc` command, which is a version of the GNU Compiler Collection that supports the ARM architecture. We will use it to compile the Rust code for the Microbit.

1. Install `putty` for from [https://www.chiark.greenend.org.uk/~sgtatham/putty/](https://www.chiark.greenend.org.uk/~sgtatham/putty/)

    The `putty` package provides the `putty` command, which is a terminal emulator that we will use to communicate with the Microbit.

## Verify Connection to Microbit

1. Ensure your Microbit is connected to your computer via USB.

1. In the terminal, run the following command.

    ```sh
    probe-rs list
    ```

    You should see a list of connected probes, including the Microbit.
