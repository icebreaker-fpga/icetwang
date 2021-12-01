/*
 * Copyright (c) 2021, Piotr Esden-Tempski <piotr@esden.net>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use core::convert::TryInto;

use embedded_hal::prelude::_embedded_hal_blocking_i2c_WriteRead;

use super::i2c::I2c;

#[derive(Default)]
pub struct Imu {
    pub acc_x: u16,
    pub acc_y: u16,
    pub acc_z: u16,
    pub gyro_x: u16,
    pub gyro_y: u16,
    pub gyro_z: u16,
    pub temp: u16
}

impl Imu {

    pub fn read(&mut self, i2c: &mut I2c) {
        let mut buffer = [0u8; 14];

        i2c.write_read(0x69, &[0x2D], &mut buffer).unwrap();

        self.acc_x = u16::from_be_bytes(buffer[0..2].try_into().unwrap());
        self.acc_y = u16::from_be_bytes(buffer[2..4].try_into().unwrap());
        self.acc_z = u16::from_be_bytes(buffer[4..6].try_into().unwrap());
        self.gyro_x = u16::from_be_bytes(buffer[6..8].try_into().unwrap());
        self.gyro_y = u16::from_be_bytes(buffer[8..10].try_into().unwrap());
        self.gyro_z = u16::from_be_bytes(buffer[10..12].try_into().unwrap());
        self.temp = u16::from_be_bytes(buffer[12..14].try_into().unwrap());
    }
}