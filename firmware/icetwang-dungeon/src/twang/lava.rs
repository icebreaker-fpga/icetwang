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

use core::time;

use super::led_string::LEDString;

const LAVA_OFF_BRIGHTNESS: u8 = 15;

#[derive(Clone, Copy)]
pub struct Lava {
    pub pos_start: i32,
    pub pos_end: i32,
    ontime: u32,
    offtime: u32,
    offset: u32,
    laston: u32,
    pub state: bool,
    pub alive: bool,}

impl Lava {
    pub fn new() -> Self {
        Self {
            pos_start: 0,
            pos_end: 0,
            ontime: 0,
            offtime: 0,
            offset: 0,
            laston: 0,
            state: false,
            alive: false
        }
    }

    pub fn draw(&self, led_string: &mut LEDString) {
        if !self.alive {
            return;
        }
        if !self.state { // Off state
            for i in self.pos_start..self.pos_end {
                let flicker = if (i % 3) == 0 {LAVA_OFF_BRIGHTNESS/2} else {LAVA_OFF_BRIGHTNESS};
                led_string[i as usize].set_rgb([LAVA_OFF_BRIGHTNESS + flicker,(LAVA_OFF_BRIGHTNESS + flicker) * 3 / 2,0]);
            }
        } else { // On state
            for i in self.pos_start..self.pos_end {
                if (i % 3) == 0 {
                    led_string[i as usize].set_rgb([150, 0, 0]);
                } else {
                    led_string[i as usize].set_rgb([180, 100, 0])
                }
            }
        }
    }

    pub fn tick(&mut self, time: u32) {
        if !self.alive {
            return;
        }
        if !self.state { // Off state
            if self.laston + self.offtime < time {
                self.state = true;
                self.laston = time;
            }
        } else {
            if self.laston + self.ontime < time {
                self.state = false;
                self.laston = time;
            }
        }
    }

    pub fn reset(&mut self) {
        self.alive = false;
    }

    pub fn spawn(&mut self, time: u32, pos_start: i32, pos_end: i32, ontime: u32, offtime: u32, offset: u32, state: bool) {
        self.pos_start = pos_start;
        self.pos_end = pos_end;
        self.ontime = ontime;
        self.offtime = offtime;
        self.offset = offset;
        self.laston = time + offset;
        self.state = state;
        self.alive = true;
    }
}