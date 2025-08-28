# Embedded Rust Hands-On

## Goal

This hands-on shall help starting a new Rust project on a micro controller that does not have an entire community behind a single core or even board.
A standard STM32 micro controller is chosen, where work is needed.
On the other side, we do not want to debug hardware problems.
For that, an eval board with spark fun sensors is likely a good choice.

The build environment shall be natively on Windows as the advantage of the better linux integration comes with a more complex hardware connection setup.

## Perquisites

### Software

- [ ] Windows system
- [ ] VS code with plugins. 
  You can install recommended plugins provided by the [extensions.json](.vscode/extensions.json)
- [ ] Git client (just to clone this repository)

### Hardware 

- [x] STM32L412KB Eval Board (Arduino Nano)
  - (Default) Connected SB18 solder bridge for Arduino Nano I2C configuration
- [x] SparkFun Qwiic adapter board (Arduino Nano)
- [x] SparkFun TPM102 temperature sensor
- [x] Qwiic cable

## Installation

### Required Tools

- Install __Rust__ and configure it for the GNU toolchain (alternatively if you have a Visual Studio Licence you may also use the default):
  To do, you can use the easy installer <https://github.com/hastur-dev/rs-easy-installer-windows>.
  which installs MSYS2 for the GNU-Toolchain and configures rust to use it as default
- Install __ARM Compiler__ (check add to path at the end) <https://developer.arm.com/-/media/Files/downloads/gnu/14.3.rel1/binrel/arm-gnu-toolchain-14.3.rel1-mingw-w64-x86_64-arm-none-eabi.exe>
- Install __ST Link__ <https://www.st.com/en/development-tools/stsw-link009.html#get-software>  
- Install __Probe-rs__ (An alternative to OpenOCD)  
  ```ps
  powershell -ExecutionPolicy Bypass -c "irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex"
  ``` 

### Configuring Rust Toolchain

- Add rust cross-compile target for stm32l4  
  
  ```ps
  rustup target add thumbv7em-none-eabihf
  ``` 

- (Optionally) Add binutils to [inspect the binary file](#inspect-binary)
  
  ```ps
  rustup component add llvm-tools-preview
  cargo install cargo-binutils
  ```

## Project Setup

Now you are ready to start your own embedded project
1. Create a new Project 
  ```ps
  cargo.exe new <Project-Name>
  ```
2. Create a rust program that prints hello world via debugger to the console
   1. Debugger Config
   2. Linker File
   3. Launch.json (+ task.json)
   4. HAL: Embassy and more (probably)
3. Add a button that controls an led
   1. 
4. Change the led to blink
   1. Add new lib: `cargo add embassy-time`
   2. Async
5. Read out the temperature of the sensor on a button press or periodically.
   1. Traits and member functions

------------------------------------------------------------------------------

## Advanced Info

### Inspect Binary

inspect the elf header

```ps
cargo readobj --bin my_nucleo_rust_sensor -- --file-headers
```

inspect linker sections

```ps
cargo size --bin my_nucleo_rust_sensor -- -A
```