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

use super::conveyor::Conveyor;
use super::lava::Lava;
use super::led_string::LEDString;
use super::utils::range_map;
use super::enemy::Enemy;

const ATTACK_WIDTH: i32 = 70;
const ATTACK_DURATION: u32 = 500;

pub struct Player {
    pub position: i32,
    direction: i32,
    pub attack_width: i32,
    pub attacking: bool,
    attacking_millis: u32,
    attack_duration: u32,
    pub speed: i32,
    pub lives: u8,
    pub alive: bool,
}

impl Player {
    pub fn new(direction: i32) -> Self {
        Self {
            position: 0,
            direction,
            attack_width: ATTACK_WIDTH,
            attacking: false,
            attacking_millis: 0,
            attack_duration: ATTACK_DURATION,
            speed: 0,
            lives: 0,
            alive: false,
        }
    }

    pub fn draw(&self, led_string: &mut LEDString, time: u32) {
        if !self.alive {
            return;
        }
        if !self.attacking {
            let pos = led_string.vtor(self.position);
            led_string[pos].set_rgb([0, 255, 0]);
        } else {
            self.draw_attack(led_string, time);
        }
    }

    fn draw_attack(&self, led_string: &mut LEDString, time: u32) {
        // Fade value
        let mut n = range_map(time - self.attacking_millis, 0, self.attack_duration, 100, 5) as u8;
        // Draw blue attack bar
        let from = led_string.vtor(self.position - (self.attack_width / 2)) + 1;
        let to = led_string.vtor(self.position + (self.attack_width / 2));
        for i in from..to {
            if i >= 0 && i < led_string.len() {
                led_string[i].set_rgb([0, 0, n]);
            }
        }
        // Draw player as white at the beginning of the attack and then back to green
        let pos = led_string.vtor(self.position);
        if n > 90 {
            n = 255;
            led_string[pos].set_rgb([255, 255, 255]);
        } else {
            n = 0;
            led_string[pos].set_rgb([0, 255, 0]);
        }
        led_string[from - 1].set_rgb([n, n, 255]);
        led_string[to].set_rgb([n, n, 255]);
    }

    pub fn tick(&mut self, led_string: &LEDString, time: u32) {
        if !self.alive {
            return;
        }
        if self.attacking {
            if self.attacking_millis + self.attack_duration < time {
                self.attacking = false;
            }
            return;
        }
        let amount = self.speed * self.direction;
        let len = led_string.vlen();
        self.position += amount;
        if self.position < 0 {
            self.position = 0
        } else if self.position >= len {
            self.position = len - 1
        }
    }

    pub fn collide_enemy(&mut self, enemy: &Enemy) {
        if !enemy.alive {
            return;
        }
        if ((enemy.player_side == 1) && (self.position >= enemy.position)) ||
            ((enemy.player_side == -1) && (self.position <= enemy.position)) {
            self.die();
        }
    }

    pub fn collide_lava(&mut self, lava: &Lava) {
        if !lava.alive || !lava.state {
            return;
        }
        if (self.position >= lava.pos_start) &&
            (self.position < lava.pos_end) {
            self.die();
        }
    }

    pub fn collide_conveyor(&mut self, conveyor: &Conveyor) {
        if !conveyor.alive {
            return;
        }
        if (self.position >= conveyor.pos_start) &&
            (self.position < conveyor.pos_end) {
                self.position += conveyor.speed;
        }
    }

    pub fn die(&mut self) {
        self.lives -= 1;
        self.alive = false;
    }

    pub fn attack(&mut self, time: u32) {
        self.attacking_millis = time;
        self.attacking = true;
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.attack_width = ATTACK_WIDTH;
        self.attack_duration = ATTACK_DURATION;
        self.alive = false;
    }

    pub fn spawn(&mut self, position: i32) {
        self.position = position;
        self.alive = true;
    }
}
