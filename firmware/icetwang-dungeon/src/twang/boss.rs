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

use super::{led_string::LEDString, player::Player, spawner::Spawner};

const BOSS_COLOR: [u8; 3] = [0x37, 0x00, 0x00]; // DarkRed
const BOSS_WIDTH: i32 = 40;
const BOSS_LIVES: usize = 3;

#[derive(Copy, Clone)]
pub struct Boss {
    pub position: i32,
    pub start: i32,
    pub stop: i32,
    lives: u8,
    live_pos: [i32; BOSS_LIVES],
    live_spawn_rate: [u32; BOSS_LIVES],
    pub alive: bool,
    pub defeated: bool,
}

impl Boss {
    pub fn new() -> Self {
        Self {
            position: 0,
            start: 0,
            stop: 0,
            lives: 3,
            live_pos: [0, 0, 0],
            live_spawn_rate: [0, 0, 0],
            alive: false,
            defeated: false,
        }
    }

    pub fn draw(&self, led_string: &mut LEDString) {
        if !self.alive {
            return;
        }
        let start = led_string.vtor(self.start);
        let stop = led_string.vtor(self.stop) + 1;
        for i in start..stop {
            led_string[i].set_rgb(BOSS_COLOR);
        }
    }

    pub fn collide_player(&mut self, player: &Player, spawners: &mut [Spawner; 2], time: u32) {
        if !self.alive {
            return;
        }
        if !player.attacking {
            return;
        }
        let pmin = player.position - (player.attack_width / 2);
        let pmax = pmin + player.attack_width;
        let bmin = self.start;
        let bmax = self.stop;
        if ((pmax >= bmin) && (pmax <= bmax)) ||
        ((pmin <= bmax) && (pmin >= bmin)) {
            self.hit(spawners, time);
        }
    }

    pub fn hit(&mut self, spawners: &mut [Spawner; 2], time: u32) {
        self.lives -= 1;
        if self.lives == 0 {
            self.alive = false;
            self.defeated = true;
        } else {
            self.do_move(spawners, time);
        }
    }

    pub fn do_move(&mut self, spawners: &mut [Spawner; 2], time: u32) {
        self.position = self.live_pos[self.lives as usize - 1];
        self.start = self.position - (BOSS_WIDTH / 2);
        self.stop = self.position + (BOSS_WIDTH / 2);
        for i in 0..spawners.len() {
            spawners[i].reset();
        }
        let rate = self.live_spawn_rate[self.lives as usize - 1];
        spawners[0].spawn(time, self.start, rate, -3, 0);
        spawners[1].spawn(time, self.stop, rate, 3, 0);
    }

    pub fn reset(&mut self) {
        self.alive = false;
        self.defeated = false;
    }

    pub fn spawn(&mut self, time: u32, positions: [i32; BOSS_LIVES], spawn_rates: [u32; BOSS_LIVES], spawners: &mut [Spawner; 2]) {
        self.live_pos = positions;
        self.live_spawn_rate = spawn_rates;
        self.lives = BOSS_LIVES as u8;
        self.alive = true;
        self.do_move(spawners, time);
    }
}