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


use icetwang_pac::I2C;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead, SevenBitAddress};

struct I2cResult {
    ack: bool,
    data: u8,
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum I2CError {
    /// No ack received
    Acknowledge,
}

pub struct I2c {
    registers: I2C,
}

impl Write<SevenBitAddress> for I2c
{
    type Error = I2CError;
    fn write(&mut self, addr: u8, output: &[u8]) -> Result<(), Self::Error> {
        self.istart();
        if self.iwrite(addr << 1) {
            return Err(I2CError::Acknowledge)
        }
        for byte in output {
            if self.iwrite(*byte) {
                return Err(I2CError::Acknowledge)
            }
        }
        self.istop();
        Ok(())
    }
}

impl Read<SevenBitAddress> for I2c {
    type Error = I2CError;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.istart();
        if self.iwrite((address << 1) | 1) {
            return Err(I2CError::Acknowledge)
        }
        if buffer.len() > 0 {
            let blen = buffer.len() - 1;
            for byte in buffer[0..blen].iter_mut() {
                *byte = self.iread(false);
            }
            buffer[blen] = self.iread(true);
        }
        self.istop();
        Ok(())
    }
}

impl WriteRead<SevenBitAddress> for I2c {
    type Error = I2CError;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.istart();
        if self.iwrite(address << 1) {
            return Err(I2CError::Acknowledge)
        }
        for byte in bytes {
            if self.iwrite(*byte) {
                return Err(I2CError::Acknowledge)
            }
        }
        self.istart();
        if self.iwrite((address << 1) | 1) {
            return Err(I2CError::Acknowledge)
        }
        if buffer.len() > 0 {
            let blen = buffer.len() - 1;
            for byte in buffer[0..blen].iter_mut() {
                *byte = self.iread(false);
            }
            buffer[blen] = self.iread(true);
        }
        self.istop();
        Ok(())
    }
}

#[allow(dead_code)]
impl I2c {

    pub fn new(registers: I2C) -> Self {
        Self { registers }
    }

    fn iwait(&mut self) -> I2cResult {
        loop {
            let v = self.registers.dat.read();
            if v.ready().bit_is_set() {
                return I2cResult {ack: v.ack().bit_is_set(), data: v.data().bits()}
            }
        }
    }

    fn istart(&mut self) {
        self.registers.dat.write(|w| w.cmd().start());
        self.iwait();
    }

    fn istop(&mut self) {
        self.registers.dat.write(|w| w.cmd().stop());
        self.iwait();
    }

    fn iwrite(&mut self, data: u8) -> bool {
        self.registers.dat.write(|w| unsafe { w.cmd().write().data().bits(data) });
        self.iwait().ack
    }

    fn iread(&mut self, ack: bool) -> u8 {
        self.registers.dat.write(|w| if ack {
            w.cmd().read().ack().set_bit()
        } else {
            w.cmd().read().ack().clear_bit()
        });
        self.iwait().data
    }

    pub fn read_reg(&mut self, device: u8, address: u8) -> Result<u8, I2CError> {
        let mut buf = [0u8; 1];
        self.write_read(device, &[address], &mut buf)?;
        Ok(buf[0])
    }

}