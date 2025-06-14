#![no_std]
#![no_main]

use car_controller::mpu6050::MPU6050I2c;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, Peripherals,
    gpio::{Level, Output, Speed},
    peripherals::I2C2,
    rcc::{Hse, HseMode},
    time::Hertz,
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = system_init();
    info!("Hello World!");

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    // let mpu = MPU6050I2c::new(p.I2C2, p.PB8.into(), p.PB9.into());

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;

        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}

fn system_init() -> Peripherals {
    // Initialize the system, peripherals, and any required configurations.
    // This function can be expanded to include more complex initialization logic.
    let mut config = Config::default();

    // Configure the RCC (Reset and Clock Control) for the STM32F4 series

    // Set the RCC to use the HSE (High-Speed External) crystal oscillator
    // with a frequency of 8 MHz as the clock source.
    config.rcc.hse = Some(Hse {
        freq: Hertz::mhz(8),       // Set HSE frequency to 8 MHz
        mode: HseMode::Oscillator, // Use the oscillator mode for HSE
    });

    embassy_stm32::init(config)
}
