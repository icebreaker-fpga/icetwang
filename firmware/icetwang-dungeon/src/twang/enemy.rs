/*
 * Copyright (c) 2020, Piotr Esden-Tempski <piotr@esden.net>
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

use crate::led_string::LEDString;
use super::player::Player;

#[derive(Copy, Clone)]
pub struct Enemy {
    pub position: i32,
    origin: i32,
    speed: i32,
    wobble: i32,
    pub alive: bool,
    pub player_side: i32
}

impl Enemy {
    pub fn new(position: i32, speed: i32, wobble: i32) -> Enemy {
        Enemy {
            position,
            origin: position,
            speed,
            wobble,
            alive: true,
            player_side: 1
        }
    }

    pub fn draw(&self, led_string: &mut LEDString) {
        if self.alive {
            led_string[self.position as usize].set_rgb([255, 0, 0]);
        }
    }

    pub fn tick(&mut self, led_string: &LEDString, time: u32) {
        if !self.alive {
            return;
        }
        if self.wobble != 0 {
            self.position = self.origin //+ //(libm::sinf((time as f32 / 3000.0) * self.speed as f32) * self.wobble as f32) as i32;
        } else {
            self.position += self.speed;
            if self.position >= led_string.len() as i32 || self.position < 0 {
                self.alive = false;
            }
        }
    }

    pub fn collide(&mut self, player: &Player) {
        if !self.alive {
            return;
        }
        if player.attacking {
            let amin = player.position - (player.attack_width / 2);
            let amax = player.position + (player.attack_width / 2);
            if amin < self.position && self.position < amax {
                self.alive = false;
            }
        }
    }
}