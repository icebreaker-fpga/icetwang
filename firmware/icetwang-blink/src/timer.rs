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

use icetwang_pac::TIMER;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use crate::{print, println};

pub struct Timer {
    registers: TIMER,
}

impl DelayUs<u32> for Timer {
    fn delay_us(&mut self, us: u32) {
        println!("{}", us);
        self.disable();
        self.load(us);
        self.enable_ev();
        self.enable();
        while !self.ev_n() {
            let val = self.value();
            if val != 0 {
                println!("{}", self.value());
            }
        }
        println!("wait ev");
        self.disable();
        println!("dis");
        self.ev_rst();
        println!("ev rst");
        self.disable_ev();
    }
}

impl DelayUs<u16> for Timer {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32);
    }
}

impl DelayUs<u8> for Timer {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32);
    }
}

impl DelayMs<u32> for Timer {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Timer {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u8> for Timer {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

#[allow(dead_code)]
impl Timer {
    pub fn new(registers: TIMER) -> Self {
        Self { registers }
    }

    pub fn enable(&mut self) {
        self.registers.csr.modify(|_, w| w.en().set_bit());
    }

    pub fn disable(&mut self) {
        self.registers.csr.modify(|_, w| w.en().clear_bit());
    }

    pub fn enable_ev(&mut self) {
        self.registers.csr.modify(|_,w| w.ev_en().set_bit());
    }

    pub fn disable_ev(&mut self) {
        self.registers.csr.modify(|_,w| w.ev_en().clear_bit());
    }

    pub fn ev_n(&mut self) -> bool {
        self.registers.csr.read().ev().bit_is_set()
    }

    pub fn ev_rst(&mut self) {
        self.registers.csr.modify(|_,w| w.ev().clear_bit());
    }

    pub fn st_n(&mut self) -> bool {
        self.registers.csr.read().st().bit_is_set()
    }

    pub fn load(&mut self, value: u32) {
        unsafe {
            self.registers.load.write(|w| w.bits(value));
        }
    }

    pub fn reload(&mut self, value: u32) {
        unsafe {
            self.registers.reload.write(|w| w.bits(value));
        }
    }

    pub fn value(&mut self) -> u32 {
        self.registers.counter.read().bits()
    }

    pub fn csr(&mut self) -> u32 {
        self.registers.csr.read().bits()
    }
}
