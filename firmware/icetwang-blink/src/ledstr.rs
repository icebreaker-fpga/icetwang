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
}