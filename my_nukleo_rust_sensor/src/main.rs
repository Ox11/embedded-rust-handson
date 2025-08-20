#![no_std]
#![no_main]

//use panic_halt as _; // Removed: using panic_probe instead

use defmt::*;
//use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_stm32::{init, rcc, Config};
use embassy_stm32::time::Hertz;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {

    let mut config = Config::default();

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

    init(config);


    info!("Hello World from a uC");
    loop {
        // your code goes here
    }
}