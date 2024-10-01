# Install Rust and Visual Studio Code

The following instructions are for setting up a student machine for the course. There are instructions for [Linux](#linux), [macOS](#macos), [Windows](#windows), and [Windows WSL 2](#windows-wsl-2). Also, [all students](#all-students) will need to create a GitHub account.

These instructions provide a minimal setup for the course. During the course other software and code libraries will be installed. Students will need local admin and network access on their machines.

These instructions will ensure you have Rust installed, Visual Studio Code installed, a Rust debugging environment, and the ability to install crates with Cargo.

Complete the setup appropriate for your machine. We will be using a tool named Valgrind in the class. Valgrind does not work on Windows. If you are using Windows, you will need to use Windows Subsystem for Linux 2 (WSL 2) to run Valgrind.

- Jump to: [Linux](#linux)
- Jump to: [macOS](#macos)
- Jump to: [Windows](#windows)
- Jump to: [Windows WSL 2](#windows-wsl-2)

During the course additional Rust tools and crates will be installed. Also, depending on your operating system additional software may be required. Students will need local admin and network access on their machines.

Also, for courses which include bare metal programming, each student will need a Microbit V2. The Microbit V2 is a small microcontroller board that can be programmed in Rust. The Microbit V2 can be purchased from various online retailers.

- [Amazon](https://www.amazon.com/Vis-Viva-BBC-Micro-Starter/dp/B0BMPDJ6XD)

Before purchasing, verify if your employer or the trainer will be providing the device for you. The link above is NOT an Amazon affiliate link.

## All Students

1. To access the courseware for the class, all students will need to create a GitHub account. If you do not already have a GitHub account, please create one at [https://www.github.com](https://www.github.com). Please know your GitHub username and password on the first day of class.

    **Note:** GitHub Enterprise accounts will not work. You must have a personal, public GitHub account.

## Linux

1. Open a terminal window, and run the following command to install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Close all terminal windows.

1. Open a new terminal window, and create a new Rust project in the home folder.

    ```sh
    cargo new test_config
    ```

1. Change to the new project folder.

    ```sh
    cd test_config
    ```

1. Build the project.

    ```sh
    cargo build
    ```

1. Run the project.

    ```sh
    cargo run
    ```

1. Verify you can install crates with Cargo. The installation should proceed without error.

    ```sh
    cargo add serde
    ```

1. Download the appropriate Visual Studio Code installer for your Linux distribution: [https://code.visualstudio.com/download](https://code.visualstudio.com/download)

1. Install Visual Studio Code on Linux.

1. Open Visual Studio Code.

1. Using the main menu, under the `File` menu, select `Open Folder...` and open the `test_config` folder.

1. On the left-hand side of Visual Studio Code, there is a vertical Activity Bar with icons. Click on the squares icon to open the Extensions view.

1. At the top of the Extensions view you will see a search box. In the extension search box, search for the `rust-lang.rust-analyzer` and install it. Then, search for the `vadimcn.vscode-lldb` extension and install it.

1. Once installed, you should be able to click `Run` and `Debug` in Visual Studio Code above the `main` function of the `test_config` project.

## macOS

1. Open a terminal window, and install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Close all terminal windows.

1. Open a new terminal window, and create a new Rust project in the home folder.

    ```sh
    cargo new test_config
    ```

1. Change to the new project folder.

    ```sh
    cd test_config
    ```

1. Build the project.

    ```sh
    cargo build
    ```

1. Run the project.

    ```sh
    cargo run
    ```

1. Verify you can install crates with Cargo. The installation should proceed without error.

    ```sh
    cargo add serde
    ```

1. Download the Visual Studio Code installer: [https://code.visualstudio.com/sha/download?build=stable&os=darwin-universal](https://code.visualstudio.com/sha/download?build=stable&os=darwin-universal)

1. Install Visual Studio Code on macOS.

1. Open Visual Studio Code.

1. Using the main menu, under the `File` menu, select `Open Folder...` and open the `test_config` folder.

1. On the left-hand side of Visual Studio Code, there is a vertical Activity Bar with icons. Click on the squares icon to open the Extensions view.

1. At the top of the Extensions view you will see a search box. In the extension search box, search for the `rust-lang.rust-analyzer` and install it. Then, search for the `vadimcn.vscode-lldb` extension and install it.

1. Once installed, you should be able to click `Run` and `Debug` in Visual Studio Code above the `main` function of the `test_config` project.

## Windows

1. Download the Rust installer: [https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe).

1. Run the `rustup-init.exe` file.

1. It may prompt you to install the Microsoft C++ build tools. If so, please install then proceded with the Rust installation.

1. Close all terminal windows, and open a new PowerShell or Windows Command Prompt terminal window, and create a new Rust project in the home folder.

    ```sh
    cargo new test_config
    ```

1. Change to the new project folder.

    ```sh
    cd test_config
    ```

1. Build the project.

    ```sh
    cargo build
    ```

1. Run the project.

    ```sh
    cargo run
    ```

1. Verify you can install crates with Cargo. The installation should proceed without error.

    ```sh
    cargo add serde
    ```

1. Download the appropriate Visual Studio Code installer: [https://code.visualstudio.com/download](https://code.visualstudio.com/download)

1. Install Visual Studio Code on Windows.

1. Open Visual Studio Code.

1. Using the main menu, under the `File` menu, select `Open Folder...` and open the `test_config` folder.

1. On the left-hand side of Visual Studio Code, there is a vertical Activity Bar with icons. Click on the squares icon to open the Extensions view.

1. At the top of the Extensions view you will see a search box. In the extension search box, search for the `rust-lang.rust-analyzer` and install it. Then, search for the `vadimcn.vscode-lldb` extension and install it.

1. Once installed, you should be able to click `Run` and `Debug` in Visual Studio Code above the `main` function of the `test_config` project.

## Windows WSL 2

1. Install and Configure WSL 2: [https://docs.microsoft.com/en-us/windows/wsl/install](https://docs.microsoft.com/en-us/windows/wsl/install)

    - **Note:** Install and configure WSL 2 - not WSL 1.

1. Install the latest Ubuntu LTS from the Microsoft Store.

1. Open a Windows WSL 2 terminal window, and install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Close all terminal windows.

1. Open a new WSL 2 terminal window, and create a new Rust project in the home folder.

    ```sh
    cargo new test_config
    ```

1. Change to the new project folder.

    ```sh
    cd test_config
    ```

1. Build the project.

    ```sh
    cargo build
    ```

1. Run the project.

    ```sh
    cargo run
    ```

1. Verify you can install crates with Cargo. The installation should proceed without error.

    ```sh
    cargo add serde
    ```

1. On Windows (not within WSL), download the appropriate Visual Studio Code installer: [https://code.visualstudio.com/download](https://code.visualstudio.com/download)

1. Install Visual Studio Code on Windows.

1. Open Visual Studio Code.

1. On the left-hand side of Visual Studio Code, there is a vertical Activity Bar with icons. Click on the squares icon to open the Extensions view.

1. At the top of the Extensions view you will see a search box. In the extension search box, search for the extension with id `ms-vscode-remote.remote-wsl`. Click the Install button.

1. Open Visual Studio Code. Open the command palette by pressing `Ctrl+Shift+P`. In the command palette, type `WSL: Open Folder` and select the option to open the folder in WSL.

1. Select the `test_config` folder in the WSL environment.

1. Install the `rust-lang.rust-analyzer` and `vadimcn.vscode-lldb` extensions in the WSL environment.

1. Once installed, you should be able to click `Run` and `Debug` in Visual Studio Code above the `main` function of the `test_config` project.
