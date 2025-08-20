# Embedded Rust Hands-On

## TODO

- [ ] Installation Guide
  - [ ] On Windows
    - <https://www.rust-lang.org/tools/install>
    - <https://visualstudio.microsoft.com/visual-cpp-build-tools/>
- [ ] Project Setup
  - [ ] Cargo new  
    ```ps
    cargo.exe new [OPTIONS] <PATH>
    ```
  - [ ] Launch.json
- [ ] VS-Code Extensions in file 

## Goal

This hands-on shall help starting a new Rust project on a micro controller that does not have an entire community behind a single core or even board.
A standard STM32 micro controller is chosen, where work is needed.
On the other side, we do not want to debug hardware problems.
For that an eval board with spark fun sensors is likely a good trade off.

## Perquisites

- [x] Windows PC
- [ ] VS code
- [x] STM32L412KB Eval Board (Arduino Nano)
  - [ ] Connected SB18 solder bridge for Arduino Nano I2C configuration
- [x] Sparkfun Qwiic Adapter board (Arduino Nano)
- [x] Sparkfun TPM102 Temperature Sensor
- [x] Qwiic Cable
- [ ] WSL Enabled
- [ ] Cargo inside WSL
- [ ] Optional USBIPD

## Project Setup

1. Create a rust program that prints hello world via debugger to the console
   1. Debugger Config
   2. Linker File
   3. Launch.json
   4. HAL: Embassy and more (probably)
2. Add a button that controls an led
   1. 
3. Change the led to blink
   1. Async
4. Read out the temperature of the sensor on a button press or periodically.
   1. Traits and member functions