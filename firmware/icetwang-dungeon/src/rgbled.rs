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

use icetwang_pac::RGBLED;

pub struct RGBLed {
    registers: RGBLED,
}

#[allow(dead_code)]
impl RGBLed {
    pub fn new(registers: RGBLED) -> Self {
        registers.pwrr.reset();
        registers.pwrg.reset();
        registers.pwrg.reset();

        registers.bcrr.reset();
        registers.bcfr.reset();

        registers.onr.reset();
        registers.ofr.reset();

        registers.br.write(|w| unsafe{ w.bits(0xE0) });

        registers.cr0.write_with_zero(|w| {
            w.fr250().set_bit();
            w.outskew().set_bit();
            w.quick_stop().set_bit();
            w.pwm_mode().set_bit();
            unsafe { w.brmsbext().bits(0x1) }
        });

        registers.csr.write_with_zero(|w| {
            w.leddexe().set_bit();
            w.rgbleden().set_bit();
            w.curren().set_bit()
        });

        Self { registers }
    }

    pub fn color(&mut self, red: u32, green: u32, blue: u32 ) {
        self.registers.pwrr.write(|w| unsafe {w.bits(red)});
        self.registers.pwrg.write(|w| unsafe {w.bits(green)});
        self.registers.pwrb.write(|w| unsafe {w.bits(blue)});
    }

    pub fn state(&mut self, on: bool) {
        self.registers.cr0.write_with_zero(|w| {
            w.ledden().bit(on);
            w.fr250().set_bit();
            w.outskew().set_bit();
            w.quick_stop().set_bit();
            w.pwm_mode().set_bit();
            unsafe { w.brmsbext().bits(0x1) }
        });
    }

    pub fn blink(&mut self, enabled: bool, on_time_ms: u32, off_time_ms: u32) {
        /* Disable EXE before doing any change. */
        self.registers.csr.write_with_zero(|w| {
            w.leddexe().clear_bit();
            w.rgbleden().set_bit();
            w.curren().set_bit()
        });

        if enabled {
            self.registers.onr.write(|w| unsafe {
                w.bits((on_time_ms >> 5) & 0xFF)
            });
            self.registers.ofr.write(|w| unsafe {
                w.bits((off_time_ms >> 5) & 0xFF)
            });
        } else {
            self.registers.onr.write(|w| unsafe {
                w.bits(0)
            });
            self.registers.ofr.write(|w| unsafe {
                w.bits(0)
            });
        }

        /* Re-enable execution. */
        self.registers.csr.write_with_zero(|w| {
            w.leddexe().set_bit();
            w.rgbleden().set_bit();
            w.curren().set_bit()
        });
    }

    pub fn breathe(&mut self, enabled: bool, rise_time_ms: u8, fall_time_ms: u8) {
        if enabled {
            self.registers.bcrr.write_with_zero(|w| {
                w.bon().set_bit();
                w.bmode().set_bit();
                unsafe {w.brate().bits((rise_time_ms >> 7) & 0x0F)}
            });
            self.registers.bcfr.write_with_zero(|w| {
                w.bon().set_bit();
                w.bmode().set_bit();
                unsafe {w.brate().bits((fall_time_ms >> 7) & 0x0F)}
            });
        } else {
            self.registers.bcrr.reset();
            self.registers.bcfr.reset();
        }
    }
}
