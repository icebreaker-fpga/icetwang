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
