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

use super::{led_string::LEDString, utils::range_map};
//use crate::print;

const CONVEYOR_BRIGHTNESS: u8 = 40;

#[derive(Clone, Copy)]
pub struct Conveyor {
    pub pos_start: i32,
    pub pos_end: i32,
    pub speed: i32,
    pub alive: bool,
}

impl Conveyor {
    pub fn new() -> Self {
        Self {
            pos_start: 0,
            pos_end: 0,
            speed: 0,
            alive: false,
        }
    }

    pub fn draw(&self, led_string: &mut LEDString, time: u32) {
        if !self.alive {
            return;
        }

        let time = time + 10000;
        let start = led_string.vtor(self.pos_start);
        let end = led_string.vtor(self.pos_end);
        for i in start..end {
            let n = ((if self.speed >= 0 {-i} else {i} + (time as i32 / 100)) % 5) as u8;
            let b = range_map(n, 0, 5, 0, CONVEYOR_BRIGHTNESS);
            //print!("{} {} ", n, b);
            if b > 0 {
                led_string[i].set_rgb([0, 0, b]);
            }
        }
    }

    pub fn reset(&mut self) {
        self.alive = false;
    }

    pub fn spawn(&mut self, pos_start: i32, pos_end: i32, speed: i32) {
        self.pos_start = pos_start;
        self.pos_end = pos_end;
        self.speed = speed;
        self.alive = true;
    }
}