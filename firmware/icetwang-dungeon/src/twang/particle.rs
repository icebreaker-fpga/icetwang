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

use super::{led_string::{LEDString, LED}, rand::random8};

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    position: i32,
    power: u8,
    life: u8,
    alive: bool,
    speed: i8
}

impl Particle {
    pub fn new() -> Self {
        Self {
            position: 0,
            power: 0,
            life: 0,
            alive: false,
            speed: 0
        }
    }

    /// Returns true if still active
    pub fn draw(&self, led_string: &mut LEDString) -> bool {
        if !self.alive {
            return false;
        }
        let pos = led_string.vtor(self.position);
        if self.power < 5 {
            let brightness = (5 - self.power) * 10;
            led_string[pos] += LED::new([brightness, brightness / 2, brightness / 2]);
            false
        } else {
            led_string[pos] += LED::new([self.power, 0, 0]);
            true
        }
    }

    pub fn tick(&mut self, gravity: bool, bend: i32) {
        if !self.alive {
            return;
        }

        // Increment life
        self.life = self.life.wrapping_add(1);

        // decrement speed based on life
        if self.speed > 0 {
            self.speed -= (self.life / 10) as i8;
        } else {
            self.speed += (self.life / 10) as i8;
        }

        // apply gravity if present
        if gravity && self.position > bend {
            self.speed -= 10;
        }

        // decrement power based on life
        self.power = 100_u8.wrapping_sub(self.life);
        if self.power == 0 {
            self.alive = false;
            return;
        }

        // Calculate new position and bounce off the end of field
        self.position += self.speed as i32 / 7;
        if self.position > 999 {
            self.position = 999;
            self.speed = -(self.speed / 2);
        } else if self.position < 0 {
            self.position = 0;
            self.speed = -(self.speed / 2);
        }
    }

    // pub fn reset(&mut self) {
    //     self.alive = false;
    // }

    pub fn spawn(&mut self, position: i32) {
        self.position = position;
        self.speed = ((random8() as i32) - 128) as i8;
        self.power = 255;
        self.alive = true;
        self.life = 220 - self.speed.abs() as u8;
    }
}