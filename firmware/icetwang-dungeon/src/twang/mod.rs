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

mod led_string;
mod utils;
mod screensaver;
mod world;
mod player;
mod enemy;
mod spawner;
mod lava;

use world::World;
use led_string::LEDString;
use super::print;

const GAME_FPS: u32 = 60;
const GAME_TIMEOUT: u32 = 60;

pub struct Twang {
    led_string: LEDString,
    screensaver: screensaver::Screensaver,
    screensaver_running: bool,
    world: World,
    input_idle_time: u32,
    level: u32,
}

impl Twang {
    pub fn new() -> Twang {
        Twang {
            led_string: LEDString::new([0,0,0]),
            screensaver: screensaver::Screensaver::new(56143584),
            screensaver_running: true,
            world: World::new(),
            input_idle_time: GAME_FPS * GAME_TIMEOUT,
            level: 0,
        }
    }

    pub fn cycle(&mut self, lr_input: i32, fire_input: bool, time: u32) {
        // Check input idle
        if lr_input == 0 && !fire_input {
            self.input_idle_time += 1;
        } else {
            self.input_idle_time = 0;
        }
        self.screensaver_running = self.input_idle_time > (GAME_FPS * GAME_TIMEOUT);

        if self.screensaver_running {
            self.level = 0;
            self.build_level(time);
            self.screensaver.tick(&mut self.led_string, time);
        } else {
            print!("LVL {} ", self.level);
            if self.world.exit_n() {
                self.level += 1;
                self.build_level(time)
            } else {
                if fire_input {
                    self.world.player_attack(time);
                }
                self.world.player_set_speed(lr_input);
                self.led_string.clear();
                self.world.tick(&mut self.led_string, time);
                self.world.collide();
                self.world.draw(&mut self.led_string, time);
            }
        }
    }

    pub fn get_raw_led(&mut self, i: usize) -> [u8; 3] {
        let led = self.led_string.get_raw(i as usize);
        [led.r, led.g, led.b]
    }

    pub fn get_raw_led_len(&mut self) -> usize {
        self.led_string.raw_len()
    }

    fn build_level(&mut self, time: u32) {
        self.world.reset();
        match self.level {
            0 => { // One enemy, kill it
                self.world.spawn_enemy(500, 0, 0);
            },
            1 => { // One enemy, kill it, it is coming for you
                self.world.spawn_enemy(999, -1, 0);
            },
            2 => { // Spawning enemies at exit every 3 seconds
                self.world.spawn_spawner(time, 999, 3000, -2, 0);
            },
            3 => { // Lava intro
                self.world.spawn_lava(time, 400, 490, 2000, 2000, 0, false);
                self.world.spawn_enemy(350, -1, 0);
                self.world.spawn_spawner(time, 999, 5500, -3, 0)
            },
            4 => { // Two sin enemies
                self.world.spawn_enemy(700, 3, 275);
                self.world.spawn_enemy(500, 2, 250);
            },
            5 => { // Enemy swarm
                self.world.spawn_enemy(700, 3, 275);
                self.world.spawn_enemy(500, 2, 250);
                self.world.spawn_enemy(600, 3, 200);
                self.world.spawn_enemy(800, 2, 350);
                self.world.spawn_enemy(400, 3, 150);
                self.world.spawn_enemy(450, 2, 400);
            },
            _ => {
                self.level = 0;
                self.build_level(time);
            }
        }
    }
}
