/*
 * Copyright (c) 2020-2021, Piotr Esden-Tempski <piotr@esden.net>
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

use super::{lava::Lava, led_string::LEDString, player::Player, utils::sini8};

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
    pub fn new() -> Self {
        Self {
            position: 500,
            origin: 500,
            speed: 0,
            wobble: 0,
            alive: false,
            player_side: 0,
        }
    }

    pub fn draw(&self, led_string: &mut LEDString) {
        if self.alive {
            let pos = led_string.vtor(self.position);
            led_string[pos].set_rgb([255, 0, 0]);
        }
    }

    pub fn tick(&mut self, led_string: &LEDString, time: u32) {
        if !self.alive {
            return;
        }
        if self.wobble != 0 {
            self.position = self.origin + ((sini8((((time / 37) as i32 * self.speed) & 0xFF) as i8) as i32) * self.wobble) / 255;
        } else {
            self.position += self.speed;
            if self.position >= led_string.vlen() || self.position < 0 {
                self.alive = false;
            }
        }
    }

    pub fn collide_player(&mut self, player: &Player) {
        if !self.alive {
            return;
        }
        if self.player_side == 0 {
            if self.position < player.position {
                self.player_side = -1;
            } else if self.position > player.position {
                self.player_side = 1;
            }
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

    pub fn collide_lava(&mut self, lava: &Lava) {
        if !self.alive || !lava.alive || !lava.state {
            return;
        }
        if (self.position >= lava.pos_start) &&
            (self.position < lava.pos_end) {
            self.alive = false;
        }
    }

    pub fn reset(&mut self) {
        self.alive = false;
    }

    pub fn spawn(&mut self, position: i32, speed: i32, wobble: i32) {
        self.alive = true;
        self.position = position;
        self.origin = position;
        self.speed = speed;
        self.wobble = wobble;
        self.player_side = 0;
    }
}
