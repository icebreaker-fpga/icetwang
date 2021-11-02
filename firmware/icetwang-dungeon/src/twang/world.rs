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
use super::enemy::Enemy;
use super::player::Player;

pub struct World {
    player: Player,
    enemies: [Enemy; 5]
}

impl World {
    pub fn new() -> World {
        World {
            player: Player::new(1),
            enemies: [Enemy::new(100, 3, 10); 5],
        }
    }

    pub fn tick(&mut self, led_string: &LEDString, time: u32) {
        self.player.tick(&led_string, time);
        for i in 0..self.enemies.len() {
            self.enemies[i].tick(&led_string, time);
        }
    }

    pub fn collide(&mut self) {
        for i in 0..self.enemies.len() {
            self.player.collide(&self.enemies[i]);
            self.enemies[i].collide(&self.player);
        }
    }

    pub fn draw(&self, led_string: &mut LEDString, time: u32) {
        self.player.draw(led_string, time);
        for i in 0..self.enemies.len() {
            self.enemies[i].draw(led_string);
        }
    }

    pub fn player_set_speed(&mut self, val: i32) {
        self.player.speed = val;
    }

    pub fn player_attack(&mut self, time: u32) {
        self.player.attack(time);
    }
}