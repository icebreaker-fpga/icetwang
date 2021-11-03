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

use icetwang_pac::UART;

pub struct Uart {
    pub registers: Option<UART>,
}

impl Uart {
    pub fn putc(&self, c: u8) {
        match self.registers.as_ref() {
            Some(reg) =>
                // Wait until TXFULL is `0`
                //while reg.csr().read().tffull().bit_is_set() {
                //    ()
                //}
                reg.data.write(|w| unsafe {w.databyte().bits(c)}),
            None => ()
        }
    }

    pub fn set_divider(&self, divider: u16) {
        match self.registers.as_ref() {
            Some(reg) =>
                reg.csr.write(|w| unsafe {w.div().bits(divider)}),
            None => ()
        }
    }
}

use core::fmt::{Error, Write};
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

#[macro_use]
#[cfg(not(test))]
pub mod print_hardware {
    use crate::print::*;
    pub static mut SUPERVISOR_UART: Uart = Uart {
        registers: None,
    };

    pub fn set_hardware(uart: UART) {
        unsafe {
            SUPERVISOR_UART.registers = Some(uart);
        }
    }

    pub fn set_divider(divider: u16) {
        unsafe {
            SUPERVISOR_UART.set_divider(divider);
        }
    }

    #[macro_export]
    macro_rules! print
    {
        ($($args:tt)+) => ({
                use core::fmt::Write;
                unsafe {
                    let _ = write!(crate::print::print_hardware::SUPERVISOR_UART, $($args)+);
                }
        });
    }
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

