use icetwang_pac::LEDSTR;
use vcell::VolatileCell;
use core::slice;

pub struct LED {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl LED {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }
}

pub struct LEDString {
    registers: LEDSTR,
    vmem: &'static mut [VolatileCell<u32>],
}

#[allow(dead_code)]
impl LEDString {
    //const VMEM: *const () = 0x86000200 as _;

    pub fn new(registers: LEDSTR) -> Self {
        registers.csr().reset();
        registers.glob().reset();
        registers.csr().reset();

        let vmem_ptr = 0x86000800 as *mut VolatileCell<u32>;

        unsafe {
            let vmem =
                slice::from_raw_parts_mut(vmem_ptr, 512);
            Self { registers, vmem }
        }
    }

    #[inline(always)]
    fn read_led(&self, index: usize) -> u32 {
        self.vmem[index].get()
    }

    #[inline(always)]
    fn write_led(&self, index: usize, rgb: u32) {
        self.vmem[index].set(rgb);
    }

    pub fn read_rgb(&self, index: usize) -> LED {
        let val = self.read_led(index);
        let r = ((val >> 16) & 0xFF) as u8;
        let g = ((val >>  8) & 0xFF) as u8;
        let b = ((val >>  0) & 0xFF) as u8;

        LED::new(r, g, b)
    }

    pub fn write_rgb(&self, index: usize, led: LED) {
        let rgb = ((led.r as u32) << 16) |
                      ((led.g as u32) <<  8) |
                      ((led.b as u32) <<  0);
        self.write_led(index, rgb);
    }

    pub fn set_glob(&mut self, glob: u16) {
        self.registers.glob().write(|w| unsafe { w.bits(glob as u32)});
    }


    pub fn set_csr(&self, start: bool, len: u16, div: u16) {
        self.registers.csr().write_with_zero(|w| unsafe {
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
        self.registers.csr().modify(|_, w| unsafe {
            w.div().bits(div)
        });
    }

    pub fn set_len(&mut self, len: u16) {
        self.registers.csr().modify(|_, w| unsafe {
            w.len().bits(len)
        });
    }

    pub fn start(&self) {
        self.registers.csr().modify(|_, w| w.strt().set_bit());
    }

    pub fn bsy_n(&self) -> bool {
        self.registers.csr().read().bsy().bit_is_set()
    }
}