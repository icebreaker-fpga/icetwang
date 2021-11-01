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

