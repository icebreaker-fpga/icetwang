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

use crate::{print, println};

struct I2cResult {
    ack: bool,
    data: u8,
}

pub struct I2c {
    registers: I2C,
}

#[allow(dead_code)]
impl I2c {
    pub fn new(registers: I2C) -> Self {
        Self { registers }
    }

    fn wait(&mut self) -> I2cResult {
        loop {
            let v = self.registers.dat.read();
            if v.ready().bit_is_set() {
                return I2cResult {ack: v.ack().bit_is_set(), data: v.data().bits()}
            }
        }
    }

    fn start(&mut self) {
        self.registers.dat.write(|w| w.cmd().start());
        self.wait();
    }

    fn stop(&mut self) {
        self.registers.dat.write(|w| w.cmd().stop());
        self.wait();
    }

    fn write(&mut self, data: u8) -> bool {
        self.registers.dat.write(|w| unsafe { w.cmd().write().data().bits(data) });
        self.wait().ack
    }

    fn read(&mut self, ack: bool) -> u8 {
        self.registers.dat.write(|w| if ack {
            w.cmd().read().ack().set_bit()
        } else {
            w.cmd().read().ack().clear_bit()
        });
        self.wait().data
    }

    pub fn write_reg(&mut self, dev: u8, reg: u8, val: u8) {
        self.start();
        self.write(dev);
        self.write(reg);
        self.write(val);
        self.stop();
    }

    pub fn read_reg(&mut self, dev: u8, reg: u8) -> u8 {
        self.start();
        self.write(dev);
        self.write(reg);
        self.start();
        self.write(dev | 1);
        let v = self.read(true);
        self.stop();
        v
    }

    pub fn read_start(&mut self, dev: u8, reg: u8) {
        self.start();
        self.write(dev);
        self.write(reg);
        self.start();
        self.write(dev | 1);
    }

    pub fn read_continue_8(&mut self, finish: bool) -> u8 {
        let v = self.read(finish);
        if finish {self.stop()}
        v
    }

    pub fn read_continue_16(&mut self, finish: bool) -> u16 {
        let v = (self.read(false) as u16) << 8 |
                     self.read(finish) as u16;
        if finish {self.stop()}
        v
    }


}