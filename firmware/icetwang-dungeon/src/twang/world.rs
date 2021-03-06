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
use super::led_string::LEDString;
use super::enemy::Enemy;
use super::spawner::Spawner;
use super::lava::Lava;
use super::player::Player;
use super::particle::Particle;
use super::boss::Boss;

const ENEMY_POOL_COUNT: usize = 10;
const SPAWNER_POOL_COUNT: usize = 2;
const LAVA_POOL_COUNT: usize = 4;
const CONVEYOR_POOL_COUNT: usize = 2;
const PARTICLE_POOL_COUNT: usize = 40;

pub struct World {
    player: Player,
    enemies: [Enemy; ENEMY_POOL_COUNT],
    spawners: [Spawner; SPAWNER_POOL_COUNT],
    lavas: [Lava; LAVA_POOL_COUNT],
    conveyors: [Conveyor; CONVEYOR_POOL_COUNT],
    particles: [Particle; PARTICLE_POOL_COUNT],
    boss: Boss,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player::new(1),
            enemies: [Enemy::new(); ENEMY_POOL_COUNT],
            spawners: [Spawner::new(); SPAWNER_POOL_COUNT],
            lavas: [Lava::new(); LAVA_POOL_COUNT],
            conveyors: [Conveyor::new(); CONVEYOR_POOL_COUNT],
            particles: [Particle::new(); PARTICLE_POOL_COUNT],
            boss: Boss::new(),
        }
    }

    pub fn tick(&mut self, led_string: &LEDString, time: u32) {
        self.player.tick(&led_string, time);
        for i in 0..self.enemies.len() {
            self.enemies[i].tick(&led_string, time);
        }
        for i in 0..self.spawners.len() {
            self.spawners[i].tick(time, &mut self.enemies)
        }
        for i in 0..self.lavas.len() {
            self.lavas[i].tick(time);
        }
    }

    pub fn collide(&mut self, time: u32) {
        for i in 0..self.enemies.len() {
            self.player.collide_enemy(&self.enemies[i]);
            self.enemies[i].collide_player(&self.player);
        }
        for i in 0..self.lavas.len() {
            self.player.collide_lava(&self.lavas[i]);
            for j in 0..self.enemies.len() {
                self.enemies[j].collide_lava(&self.lavas[i]);
            }
        }
        for i in 0..self.conveyors.len() {
            self.player.collide_conveyor(&self.conveyors[i]);
        }
        self.boss.collide_player(&self.player, &mut self.spawners, time);
        self.player.collide_boss(&self.boss);
    }

    pub fn draw(&self, led_string: &mut LEDString, time: u32) {
        for i in 0..self.spawners.len() {
            self.spawners[i].draw(led_string);
        }

        for i in 0..self.lavas.len() {
            self.lavas[i].draw(led_string);
        }

        for i in 0..self.conveyors.len() {
            self.conveyors[i].draw(led_string, time);
        }

        self.boss.draw(led_string);

        // Enemies walk on conveyors and other stuff
        for i in 0..self.enemies.len() {
            self.enemies[i].draw(led_string);
        }

        // Player walks on everything
        self.player.draw(led_string, time);

        // Draw exit
        // You can't escape the boss so there is no exit ;)
        if !self.boss.alive {
            let exit = led_string.len() - 1;
            led_string[exit].set_rgb([0, 0, 255]);
        }
    }

    pub fn player_set_speed(&mut self, val: i32) {
        self.player.speed = val;
    }

    pub fn player_attack(&mut self, time: u32) {
        self.player.attack(time);
    }

    pub fn player_set_attack_width(&mut self, width: i32) {
        self.player.attack_width = width;
    }

    pub fn player_alive(&mut self) -> bool {
        self.player.alive
    }

    pub fn player_position(&mut self) -> i32 {
        self.player.position
    }

    pub fn player_lives(&mut self) -> u8 {
        self.player.lives
    }

    pub fn player_set_lives(&mut self, lives: u8) {
        self.player.lives = lives;
    }

    pub fn boss_killed(&self) -> bool {
        !self.boss.alive && self.boss.defeated
    }

    pub fn reset(&mut self) {
        self.player.reset();
        for i in 0..self.enemies.len() {
            self.enemies[i].reset();
        }
        for i in 0..self.spawners.len() {
            self.spawners[i].reset();
        }
        for i in 0..self.lavas.len() {
            self.lavas[i].reset();
        }
        for i in 0..self.conveyors.len() {
            self.conveyors[i].reset();
        }
        self.boss.reset();
    }

    pub fn spawn_player(&mut self, position: i32) {
        self.player.spawn(position);
    }

    pub fn spawn_enemy(&mut self, position: i32, speed: i32, wobble: i32) {
        for i in 0..self.enemies.len() {
            if self.enemies[i].alive { continue }
            else {
                self.enemies[i].spawn(position, speed, wobble);
                return;
            }
        }
    }

    pub fn spawn_spawner(&mut self, time: u32, position: i32, rate: u32, speed: i32, activate: u32) {
        for i in 0..self.spawners.len() {
            if self.spawners[i].alive { continue }
            else {
                self.spawners[i].spawn(time, position, rate, speed, activate);
                return;
            }
        }
    }

    pub fn spawn_lava(&mut self, time: u32, pos_start: i32, pos_end: i32, ontime: u32, offtime: u32, offset: u32, state: bool) {
        for i in 0..self.lavas.len() {
            if self.lavas[i].alive { continue }
            else {
                self.lavas[i].spawn(time, pos_start, pos_end, ontime, offtime, offset, state);
                return;
            }
        }
    }

    pub fn spawn_conveyor(&mut self, pos_start: i32, pos_end: i32, speed: i32) {
        for i in 0..self.conveyors.len() {
            if self.conveyors[i].alive { continue }
            else {
                self.conveyors[i].spawn(pos_start, pos_end, speed);
                return;
            }
        }
    }

    pub fn spawn_particles(&mut self, position: i32) {
        for i in 0..self.particles.len() {
            self.particles[i].spawn(position);
        }
    }

    pub fn spawn_boss(&mut self, time: u32, positions: [i32; 3], spawn_rates: [u32; 3]) {
        self.boss.spawn(time, positions, spawn_rates, &mut self.spawners);
    }

    // Returns true if still active
    pub fn cycle_particles(&mut self, led_string: &mut LEDString, gravity: bool, bend: i32) -> bool {
        let mut active = false;
        for i in 0..self.particles.len() {
            self.particles[i].tick(gravity, bend);
            if self.particles[i].draw(led_string) {
                active = true;
            }
        }
        active
    }

    pub fn exit_n(&self) -> bool {
        self.player.position == 999
    }
}
