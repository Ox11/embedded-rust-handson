#![no_std]
#![no_main]

//use panic_halt as _; // Removed: using panic_probe instead

use defmt::*;
//use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;

// Required libraries for clock tree config
use embassy_stm32::{init, rcc, Config};
use embassy_stm32::time::Hertz;

// Required libraries for Pins
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {

    let p = init_core(); // init core and get all peripherals

    info!("Hello World from my Nukleo board");
    warn!("Actually not that critical");
    error!("Actually not not even an error");

    // Init button and LED
    // Button is actually a jumper and pin D2 and GND. Hence a pull-up is required
    let jumper_btn = Input::new(p.PA12, Pull::Up);
    // led must be mutable as we would like to change its state.
    let mut led_ld3 = Output::new(p.PB3, Level::Low, Speed::Low);

    loop {
        // If the jumper is set, the input is low
        if jumper_btn.is_high()  // In rust no parentheses are required around the if clause
        {
            led_ld3.set_high();
        }
        else {
            led_ld3.set_low();
        }
    }
}

// Setup clock tree config of STM32l412 using the embassy_stm32 library.
// return an object that contains all peripherals
fn init_core() -> embassy_stm32::Peripherals {

    // create the config struct of type Config and initialize it with default values.
    let mut config = Config::default();

    // overwrite the default values. If we miss one, no problem it just has the default value.
    config.rcc = rcc::Config {
        msi: Some(rcc::MSIRange::RANGE4M),
        hsi: false,
        hse: None,
        hsi48: None,
        pll: Some(rcc::Pll {
            source: rcc::PllSource::MSI,
            mul: rcc::PllMul::MUL40,
            prediv: rcc::PllPreDiv::DIV1,
            divr: Some(rcc::PllRDiv::DIV2),
            divq: None,
            divp: None,
        }),
        pllsai1: None, // Used for USB, RNG, SDMMC
        sys:rcc::Sysclk::PLL1_R,
        ahb_pre: rcc::AHBPrescaler::DIV1,
        apb1_pre: rcc::APBPrescaler::DIV1,
        apb2_pre: rcc::APBPrescaler::DIV1,
        ls: rcc::LsConfig {
            rtc: rcc::RtcClockSource::LSE,
            lsi: false,
            lse: Some(rcc::LseConfig {
                frequency: Hertz::hz(32_768),
                mode: rcc::LseMode::Oscillator(rcc::LseDrive::MediumHigh),
            }),
        },
        mux: Default::default(),
    };

    // no ';' at the end of the last line means implicitly to return this value
    init(config) 
}
