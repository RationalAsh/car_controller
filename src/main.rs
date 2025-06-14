#![no_std]
#![no_main]

use car_controller::mpu6050::{
    MPU6050_CLOCK_PLL_XGYRO, MPU6050_RA_PWR_MGMT_1, MPU6050_RA_WHO_AM_I, MPU6050BitField,
    MPU6050I2c, MPUClkSource,
};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, Peripherals,
    gpio::{Level, Output, Speed},
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

    let mut imu_sensor = MPU6050I2c::new(p.I2C2, p.PB10, p.PB11);

    let _ = imu_sensor.write_field(MPUClkSource::PLLWithXGyro);

    loop {
        match imu_sensor.read_field::<MPUClkSource>() {
            Ok(who_am_i) => {
                info!("MPU6050 WHO_AM_I: {:#04x}", who_am_i.to_value());
            }
            Err(e) => {
                error!("Failed to read WHO_AM_I: {:?}", e);
            }
        }
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

    // embassy_stm32::pac::AFIO
    //     .mapr()
    //     .modify(|w| w.set_i2c1_remap(true));

    embassy_stm32::init(config)
}
