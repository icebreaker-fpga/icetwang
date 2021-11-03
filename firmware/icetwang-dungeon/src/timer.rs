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

pub struct Timer {
    registers: TIMER,
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
