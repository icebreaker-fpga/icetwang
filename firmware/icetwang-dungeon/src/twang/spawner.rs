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

use super::{led_string::LEDString,enemy::Enemy};

#[derive(Clone, Copy)]
pub struct Spawner {
    position: i32,      // Spawner position
    rate: u32,          // Spawn rate in ms
    speed: i32,         // Eneemy speed and direction when exiting the spawner
    last_spawned: u32,  // Time of last spawn
    activate: u32,      // Time of activation
    pub alive: bool,        // Is this spawner alive
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            position: 500,
            rate: 0,
            speed: 0,
            last_spawned: 0,
            activate: 0,
            alive: false,
        }
    }

    pub fn draw(&self, led_string: &mut LEDString) {
        if self.alive {
            led_string[self.position as usize].set_rgb([64, 0, 64]);
        }
    }

    pub fn tick(&mut self, time: u32, enemies: &mut [Enemy]) {
        if !self.alive || self.activate >= time {
            return;
        }
        if self.last_spawned + self.rate < time || self.last_spawned == 0 {
            for i in 0..enemies.len() {
                    if enemies[i].alive { continue }
                else {
                    enemies[i].spawn(self.position, self.speed, 0);
                    self.last_spawned = time;
                    return;
                }
            }
           }
    }

    pub fn reset(&mut self) {
        self.alive = false;
    }

    pub fn spawn(&mut self, time: u32, position: i32, rate: u32, speed: i32, activate: u32) {
        self.position = position;
        self.rate = rate;
        self.speed = speed;
        self.last_spawned = 0;
        self.activate = time + activate;
        self.alive = true;
    }
}
