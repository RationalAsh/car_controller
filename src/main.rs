#![no_std]
#![no_main]

use car_controller::{
    AccelFullScaleRange, GyroFullScaleRange, MPUClkSource, SleepMode, TempDisable,
    mpu6050::MPU6050I2c,
};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    Config, Peripherals,
    gpio::{Level, Output, Speed},
    rcc::{APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllSource, Sysclk},
    time::Hertz,
};
use embassy_time::{Duration, Ticker};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = system_init();
    info!("Hello World!");

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);
    let peripheral = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C2,
        p.PB10,
        p.PB11,
        Hertz::khz(400),
        embassy_stm32::i2c::Config::default(),
    );

    // let mut imu_sensor = MPU6050I2c::new(p.I2C2, p.PB10, p.PB11, Hertz::khz(400));
    let mut imu_sensor = car_controller::mpu6050v2::MPU6050::new(peripheral);

    let _ = imu_sensor.write_field(MPUClkSource::PLLWithXGyro);
    let _ = imu_sensor.write_field(TempDisable::Disable);
    let _ = imu_sensor.write_field(GyroFullScaleRange::FS500);
    let _ = imu_sensor.write_field(AccelFullScaleRange::FS2);
    let _ = imu_sensor.write_field(SleepMode::WakeUp);

    let mut ticker = Ticker::every(Duration::from_hz(50));

    loop {
        // match imu_sensor.read_field::<GyroFullScaleRange>() {
        //     Ok(gfsr) => {
        //         info!("Gyro Full Scale Range: {:?}", gfsr);
        //     }
        //     Err(e) => {
        //         error!("Error reading Gyro Full Scale Range: {:?}", e);
        //     }
        // }
        match imu_sensor.read_accel_gyro() {
            Ok((ax, ay, az, gx, gy, gz)) => {
                info!("{} {} {} {} {} {}", ax, ay, az, gx, gy, gz);
            }
            Err(e) => {
                error!("Error reading IMU data: {:?}", e);
            }
        }

        led.toggle();
        ticker.next().await;
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
