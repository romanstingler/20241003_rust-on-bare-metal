# Create Bare Metal Project

1. Create a new Rust project named `hello_microbit`.

    Bare metal projects are created using the `cargo new` command. This command creates a new Rust project with the specified name.

    ```sh
    cargo new hello_microbit
    ```

1. Change to the `hello_microbit` directory.

    ```sh
    cd hello_microbit
    ```

1. Add the following crates to the `Cargo.toml` file under the `[dependencies]` section.

    ```toml
    cortex-m = "0.7.3"
    cortex-m-rt = "0.7.0"
    panic-halt = "0.2.0"
    rtt-target = { version = "0.3.1", features = ["cortex-m"] }
    panic-rtt-target = { version = "0.1.2", features = ["cortex-m"] }
    microbit-v2 = "0.13.0"
    ```

    - `cortex-m` provides low-level access to the ARM Cortex-M processor.
    - `cortex-m-rt` provides runtime support for Cortex-M microcontrollers.
    - `panic-halt` provides a panic handler that halts the program.
    - `rtt-target` provides RTT logging support.
    - `panic-rtt-target` provides a panic handler that logs messages to RTT.
    - `microbit-v2` provides the API for the micro:bit.

1. In the root folder of the project, create a new file named `Embed.toml` and add the following content to it.

    The `Embed.toml` file specifies the default configuration for the project. The `chip` field specifies the target chip. This is required for the `cargo embed` command to know which chip to target.

    ```toml
    [default.general]
    chip = "nrf52833_xxAA"

    [default.rtt]
    enabled = true
    ```

    The `rtt` section specifies the RTT configuration. The `enabled` field specifies whether RTT logging is enabled. This is required for the `cargo embed` command to enable RTT logging. When the `enabled` field is set to `true`, RTT logging will be displayed in the terminal.

1. In the root folder of the project, create a new file named `memory.x` and add the following content to it.

    The `memory.x` file specifies the memory layout of the target device. The `FLASH` section specifies the flash memory region, and the `RAM` section specifies the RAM memory region. This is required for the linker to know where to place the code and data. The configuration is specific to the target device.

    ```text
    MEMORY
    {
      FLASH : ORIGIN = 0x00000000, LENGTH = 256K
      RAM : ORIGIN = 0x20000000, LENGTH = 16K
    }
    ```

1. In the root folder of the project, create a new file named `build.rs` and add the following Rust code to it.

    This will copy the `memory.x` file to the output directory and specify the output directory as the linker search path. In Rust, the `build.rs` file in the project's root folder is a build script that is run before the build starts. This is useful for generating code, running external tools, or setting up environment variables.

    ```rust
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    fn main() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("memory.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=memory.x");
    }
    ```

1. Create a new folder named `.cargo` in the root folder of the project.

    ```sh
    mkdir .cargo
    ```

1. In the `.cargo` folder, create a new file named `config.toml` and add the following content to it.

    ```toml
    [build]
    target = "thumbv7em-none-eabihf"

    [target.thumbv7em-none-eabihf]
    runner = "probe-rs run --chip nRF52833_xxAA"
    rustflags = ["-C", "linker=rust-lld", "-C", "link-arg=-Tlink.x"]
    ```

    - `build` section specifies the build configuration, in this case, the target architecture.
    - `target` section specifies the target architecture and the `rustflags` section specifies the linker arguments. This will target the ARM architecture and use the `link.x` file as the linker script.

1. Open the `src/main.rs` file and add the following Rust code to it.

    ```rust
    #![deny(unsafe_code)]
    #![no_main]
    #![no_std]

    // `entry` is the entry point of the program, `main` is not assumed because of `no_main`
    use cortex_m_rt::entry;

    // the `microbit` crate provides the API for the micro:bit
    // if this is omitted, the compile will fail
    use microbit as _;

    // `rtt_target` is used for RTT logging
    use rtt_target::{rtt_init_print,rprintln};

    // `panic_rtt_target` is used for panic messages
    use panic_rtt_target as _;

    #[entry]
    fn main() -> ! {
        // initialize RTT logging
        rtt_init_print!();

        loop {
            // print a message to the RTT log
            rprintln!("Hello from micro:bit!");
        }
    }
    ```

    - #![deny(unsafe_code)]: This attribute denies the use of unsafe code in the entire crate. If any unsafe code is present, the compiler will produce an error. This is used to enforce memory safety and other guarantees provided by Rust.
    - #![no_main]: This attribute indicates that the crate does not use the standard main function entry point. This is typically used in embedded systems or operating system kernels where the entry point is defined differently.
    - #![no_std]: This attribute tells the compiler that the crate will not use the Rust standard library (std). Instead, it will use the core library, which is a subset of the standard library that is suitable for environments without an operating system, such as embedded systems.

1. Build and flash the project to the Microbit.

    The `--target` option specifies the target architecture. This is required when building for a bare-metal target.

    ```sh
    cargo embed
    ```

    The message `Hello from micro:bit!` should be repeatedly displayed in the terminal.
