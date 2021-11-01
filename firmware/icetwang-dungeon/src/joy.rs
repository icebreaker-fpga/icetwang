use icetwang_pac::JOY;

#[derive(Debug)]
pub struct JoyState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

pub struct Joy {
    registers: JOY,
}

#[allow(dead_code)]
impl Joy {
    pub fn new(registers: JOY) -> Self {
        Self { registers }
    }

    pub fn get(&mut self) -> JoyState {
        let state = self.registers.joy.read().bits();
        let up    = (state & 0x01) != 0;
        let down  = (state & 0x02) != 0;
        let left  = (state & 0x04) != 0;
        let right = (state & 0x08) != 0;
        JoyState { up, down, left, right }
    }
}