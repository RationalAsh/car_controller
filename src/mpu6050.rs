use embassy_stm32::{
    i2c::{Config, I2c, Instance, SclPin, SdaPin},
    time::Hertz,
};

use super::*;

/// I2C driver for the MPU6050 sensor.
pub struct MPU6050I2c<'d> {
    peripheral: I2c<'d, embassy_stm32::mode::Blocking>,
    address: u8,
}

impl<'d> MPU6050I2c<'d> {
    pub fn new<P: Instance>(
        peri: P,
        scl_pin: impl SclPin<P>,
        sda_pin: impl SdaPin<P>,
        freq: Hertz,
    ) -> Self {
        MPU6050I2c {
            peripheral: I2c::new_blocking(peri, scl_pin, sda_pin, freq, Config::default()),
            address: MPU6050_DEFAULT_ADDRESS,
        }
    }

    pub fn new_with_address<P: Instance>(
        peri: P,
        scl_pin: impl SclPin<P>,
        sda_pin: impl SdaPin<P>,
        address: u8,
        freq: Hertz,
    ) -> Self {
        MPU6050I2c {
            peripheral: I2c::new_blocking(peri, scl_pin, sda_pin, freq, Config::default()),
            address,
        }
    }

    /// Read a byte from the specified register of the MPU6050.
    pub fn read_byte(&mut self, reg: u8) -> Result<u8, embassy_stm32::i2c::Error> {
        let mut buf = [0; 1];
        self.peripheral
            .blocking_write_read(self.address, &[reg], &mut buf)?;
        Ok(buf[0])
    }

    /// Write a byte to the specified register of the MPU6050.
    pub fn write_byte(&mut self, reg: u8, value: u8) -> Result<(), embassy_stm32::i2c::Error> {
        self.peripheral.blocking_write(self.address, &[reg, value])
    }

    pub fn read_field<T: MPU6050BitField>(&mut self) -> Result<T, embassy_stm32::i2c::Error> {
        // 1. Read the present 8-bit value in that register
        let value = self.read_byte(T::addr())?;

        // 2. Shift the bits to the right so that the field is in the least significant bits
        let shifted_value = value >> (T::location() - T::length() + 1);

        // 3. Mask the bits to get only the bits that belong to the field
        let masked_value = shifted_value & T::mask();

        // 4. Return the masked value
        Ok(T::from(masked_value))
    }

    pub fn write_field<T: MPU6050BitField>(
        &mut self,
        field: T,
    ) -> Result<(), embassy_stm32::i2c::Error> {
        // 1. Read the present 8-bit value in that register
        let mut current_value = self.read_byte(T::addr())?;

        // 2. Clear (zero) the bits that belong to the field
        current_value &= !(T::mask() << T::location());

        // 3. Insert the bits you want, lined up at the correct position
        current_value |= (field.to_value() & T::mask()) << T::location();

        // 4. Write the new byte back to the device
        self.write_byte(T::addr(), current_value)
    }

    pub fn read_accel_x(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_ACCEL_XOUT_H, MPU6050_RA_ACCEL_XOUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_ACCEL_XOUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_accel_y(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_ACCEL_YOUT_H, MPU6050_RA_ACCEL_YOUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_ACCEL_YOUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_accel_z(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_ACCEL_ZOUT_H, MPU6050_RA_ACCEL_ZOUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_ACCEL_ZOUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_temp(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_TEMP_OUT_H, MPU6050_RA_TEMP_OUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_TEMP_OUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_gyro_x(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_GYRO_XOUT_H, MPU6050_RA_GYRO_XOUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_GYRO_XOUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_gyro_y(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_GYRO_YOUT_H, MPU6050_RA_GYRO_YOUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_GYRO_YOUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_gyro_z(&mut self) -> Result<i16, embassy_stm32::i2c::Error> {
        let mut data = [MPU6050_RA_GYRO_ZOUT_H, MPU6050_RA_GYRO_ZOUT_L];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_GYRO_ZOUT_H], &mut data)?;
        Ok(((data[0] as i16) << 8) | (data[1] as i16))
    }

    pub fn read_accel(&mut self) -> Result<(i16, i16, i16), embassy_stm32::i2c::Error> {
        let mut data = [
            MPU6050_RA_ACCEL_XOUT_H,
            MPU6050_RA_ACCEL_XOUT_L,
            MPU6050_RA_ACCEL_YOUT_H,
            MPU6050_RA_ACCEL_YOUT_L,
            MPU6050_RA_ACCEL_ZOUT_H,
            MPU6050_RA_ACCEL_ZOUT_L,
        ];

        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_ACCEL_XOUT_H], &mut data)?;

        let accel_x = ((data[0] as i16) << 8) | (data[1] as i16);
        let accel_y = ((data[2] as i16) << 8) | (data[3] as i16);
        let accel_z = ((data[4] as i16) << 8) | (data[5] as i16);

        Ok((accel_x, accel_y, accel_z))
    }

    pub fn read_gyro(&mut self) -> Result<(i16, i16, i16), embassy_stm32::i2c::Error> {
        let mut data = [
            MPU6050_RA_GYRO_XOUT_H,
            MPU6050_RA_GYRO_XOUT_L,
            MPU6050_RA_GYRO_YOUT_H,
            MPU6050_RA_GYRO_YOUT_L,
            MPU6050_RA_GYRO_ZOUT_H,
            MPU6050_RA_GYRO_ZOUT_L,
        ];
        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_GYRO_XOUT_H], &mut data)?;

        let gyro_x = ((data[0] as i16) << 8) | (data[1] as i16);
        let gyro_y = ((data[2] as i16) << 8) | (data[3] as i16);
        let gyro_z = ((data[4] as i16) << 8) | (data[5] as i16);
        Ok((gyro_x, gyro_y, gyro_z))
    }

    pub fn read_accel_gyro(
        &mut self,
    ) -> Result<(i16, i16, i16, i16, i16, i16), embassy_stm32::i2c::Error> {
        let accel = self.read_accel()?;
        let gyro = self.read_gyro()?;
        let (accel_x, accel_y, accel_z) = accel;
        let (gyro_x, gyro_y, gyro_z) = gyro;

        Ok((accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z))
    }

    pub fn read_all(
        &mut self,
    ) -> Result<(i16, i16, i16, i16, i16, i16, i16), embassy_stm32::i2c::Error> {
        let mut data = [
            MPU6050_RA_ACCEL_XOUT_H,
            MPU6050_RA_ACCEL_XOUT_L,
            MPU6050_RA_ACCEL_YOUT_H,
            MPU6050_RA_ACCEL_YOUT_L,
            MPU6050_RA_ACCEL_ZOUT_H,
            MPU6050_RA_ACCEL_ZOUT_L,
            MPU6050_RA_TEMP_OUT_H,
            MPU6050_RA_TEMP_OUT_L,
            MPU6050_RA_GYRO_XOUT_H,
            MPU6050_RA_GYRO_XOUT_L,
            MPU6050_RA_GYRO_YOUT_H,
            MPU6050_RA_GYRO_YOUT_L,
            MPU6050_RA_GYRO_ZOUT_H,
            MPU6050_RA_GYRO_ZOUT_L,
        ];

        self.peripheral
            .blocking_write_read(self.address, &[MPU6050_RA_ACCEL_XOUT_H], &mut data)?;

        let accel_x = ((data[0] as i16) << 8) | (data[1] as i16);
        let accel_y = ((data[2] as i16) << 8) | (data[3] as i16);
        let accel_z = ((data[4] as i16) << 8) | (data[5] as i16);
        let temp = ((data[6] as i16) << 8) | (data[7] as i16);
        let gyro_x = ((data[8] as i16) << 8) | (data[9] as i16);
        let gyro_y = ((data[10] as i16) << 8) | (data[11] as i16);
        let gyro_z = ((data[12] as i16) << 8) | (data[13] as i16);
        Ok((accel_x, accel_y, accel_z, temp, gyro_x, gyro_y, gyro_z))
    }
}
