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

use icetwang_pac::LEDSTR;
use vcell::VolatileCell;
use core::slice;

pub struct LEDStringHAL {
    registers: LEDSTR,
    vmem: &'static mut [VolatileCell<u32>],
}

#[allow(dead_code)]
impl LEDStringHAL {
    //const VMEM: *const () = 0x86000200 as _;

    pub fn new(registers: LEDSTR) -> Self {
        registers.csr.reset();
        registers.glob.reset();
        registers.csr.reset();

        let vmem_ptr = 0x86000800 as *mut VolatileCell<u32>;

        unsafe {
            let vmem =
                slice::from_raw_parts_mut(vmem_ptr, 512);
            Self { registers, vmem }
        }
    }

    #[inline(always)]
    fn read_led(&self, index: u16) -> u32 {
        self.vmem[index as usize].get()
    }

    #[inline(always)]
    fn write_led(&self, index: u16, rgb: u32) {
        self.vmem[index as usize].set(rgb);
    }

    pub fn read_rgb(&self, index: u16) -> [u8; 3] {
        let val = self.read_led(index);
        let r = ((val >> 16) & 0xFF) as u8;
        let g = ((val >>  8) & 0xFF) as u8;
        let b = ((val >>  0) & 0xFF) as u8;

        [r, g, b]
    }

    pub fn write_rgb(&self, index: u16, led: [u8; 3]) {
        let rgb = ((led[0] as u32) << 16) |
                      ((led[1] as u32) <<  8) |
                      ((led[2] as u32) <<  0);
        self.write_led(index, rgb);
    }

    pub fn set_glob(&mut self, glob: u16) {
        self.registers.glob.write(|w| unsafe { w.bits(glob as u32)});
    }


    pub fn set_csr(&self, start: bool, len: u16, div: u16) {
        self.registers.csr.write_with_zero(|w| unsafe {
            w.div().bits(div);
            w.len().bits(len);
            if start {
                w.strt().set_bit()
            } else {
                w.strt().clear_bit()
            }
        });
    }

    pub fn set_div(&mut self, div: u16) {
        self.registers.csr.modify(|_, w| unsafe {
            w.div().bits(div)
        });
    }

    pub fn set_len(&mut self, len: u16) {
        self.registers.csr.modify(|_, w| unsafe {
            w.len().bits(len)
        });
    }

    pub fn start(&self) {
        self.registers.csr.modify(|_, w| w.strt().set_bit());
    }

    pub fn bsy_n(&self) -> bool {
        self.registers.csr.read().bsy().bit_is_set()
    }
}
