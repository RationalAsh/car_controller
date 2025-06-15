#![no_std]
#![no_main]

use car_controller::mpu6050::{MPU6050BitField, MPU6050I2c, MPUClkSource, SleepMode, TempDisable};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, Peripherals,
    gpio::{Level, Output, Speed},
    rcc::{APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllSource, Sysclk},
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
    let _ = imu_sensor.write_field(TempDisable::Disable);
    let _ = imu_sensor.write_field(SleepMode::WakeUp);

    loop {
        led.set_high();
        match imu_sensor.read_accel() {
            Ok(accel) => {
                info!("Accel X: {}, Y: {}, Z: {}", accel.0, accel.1, accel.2);
            }
            Err(e) => {
                error!("Failed to read accelerometer: {:?}", e);
            }
        }

        match imu_sensor.read_gyro() {
            Ok(gyro) => {
                info!("Gyro X: {}, Y: {}, Z: {}", gyro.0, gyro.1, gyro.2);
            }
            Err(e) => {
                error!("Failed to read gyroscope: {:?}", e);
            }
        }
        // match imu_sensor.read_gyro_z() {
        //     Ok(gyro_z) => {
        //         info!("Gyro Z: {}", gyro_z);
        //     }
        //     Err(e) => {
        //         error!("Failed to read gyro Z: {:?}", e);
        //     }
        // }

        led.set_low();
        Timer::after_millis(10).await;
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

    config.rcc.pll = Some(Pll {
        src: PllSource::HSE,
        prediv: PllPreDiv::DIV1, // HSE clock divided by 1
        mul: PllMul::MUL9,
    });

    config.rcc.sys = Sysclk::PLL1_P; // Use PLL as the system clock source
    config.rcc.apb1_pre = APBPrescaler::DIV2; // Set APB1 prescaler to 4
    config.rcc.apb2_pre = APBPrescaler::DIV1; // Set APB2 prescaler to 1

    // embassy_stm32::pac::AFIO
    //     .mapr()
    //     .modify(|w| w.set_i2c1_remap(true));

    embassy_stm32::init(config)
}
