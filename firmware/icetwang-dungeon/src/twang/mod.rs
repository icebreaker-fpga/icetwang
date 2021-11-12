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
mod conveyor;
mod rand;

use world::World;
use led_string::LEDString;
use self::rand::random8lim;

#[cfg(feature = "icetwanghw")]
use super::print;
use utils::range_map;

const GAME_FPS: u32 = 60;
const GAME_TIMEOUT: u32 = 60;
const STARTUP_WIPEUP_DUR: u32 = 200;
const STARTUP_SPARKLE_DUR: u32 = 1300;
const STARTUP_FADE_DUR: u32 = 1500;

#[derive(Clone, Copy)]
enum StartStage {
    Wipeup,
    Sparkle,
    Fade,
}

enum State {
    Screensaver,
    Starting{stage: StartStage, start_time: u32, },
    Playing,
    Death,
    Lives,
    Win,
}

pub struct Twang {
    led_string: LEDString,
    screensaver: screensaver::Screensaver,
    state: State,
    world: World,
    input_idle_time: u32,
    level: u32,
}

impl Twang {
    pub fn new() -> Twang {
        Twang {
            led_string: LEDString::new(),
            screensaver: screensaver::Screensaver::new(),
            state: State::Screensaver,
            world: World::new(),
            input_idle_time: GAME_FPS * GAME_TIMEOUT,
            level: 0,
        }
    }

    pub fn cycle(&mut self, lr_input: i32, fire_input: bool, time: u32) {

        self.state = match self.state {
            State::Screensaver => {
                self.screensaver.tick(&mut self.led_string, time);
                if lr_input != 0 || fire_input {
                    self.input_idle_time = 0;
                    State::Starting{stage: StartStage::Wipeup, start_time: time}
                } else {
                    State::Screensaver
                }
            },
            State::Starting {stage, start_time} => {
                self.led_string.clear();
                match stage {
                    StartStage::Wipeup => {
                        let n = range_map(time - start_time, 0, STARTUP_WIPEUP_DUR, 0, self.led_string.len() as u32) as i32;
                        for i in 0..n {
                            self.led_string[i].set_rgb([0, 255, 0])
                        }
                        if time < (start_time + STARTUP_WIPEUP_DUR) {
                            State::Starting{stage: StartStage::Wipeup, start_time: start_time}
                        } else {
                            State::Starting{stage: StartStage::Sparkle, start_time: time}
                        }
                    },
                    StartStage::Sparkle => {
                        // we need rand to sparkle
                        for i in 0..self.led_string.len() {
                            if random8lim(30) < 28 {
                                self.led_string[i].set_rgb([0, 255, 0]);
                            } else {
                                let flicker = random8lim(250);
                                self.led_string[i].set_rgb([flicker, 150, flicker]);
                            }
                        }
                        if time < (start_time + STARTUP_SPARKLE_DUR) {
                            State::Starting{stage: StartStage::Sparkle, start_time: start_time}
                        } else {
                            State::Starting{stage: StartStage::Fade, start_time: time}
                        }
                    },
                    StartStage::Fade => {
                        let n = range_map(time - start_time, 0, STARTUP_FADE_DUR, 0, self.led_string.len() as u32) as i32;
                        let brightness = range_map(time - start_time, 0, STARTUP_FADE_DUR, 255, 0) as u8;
                        //println!("st{} t{} td{} n{} b{}", start_time, time, time-start_time, n, brightness);
                        for i in n..self.led_string.len() {
                            self.led_string[i].set_rgb([0, brightness, 0]);
                        }
                        if time < (start_time + STARTUP_FADE_DUR) {
                            State::Starting{stage: StartStage::Fade, start_time: start_time}
                        } else {
                            self.level = 0;
                            self.build_level(time);
                            State::Playing
                        }
                    }
                }
            },
            State::Playing => {
                print!("LVL {} ", self.level);
                if self.world.exit_n() {
                    self.level += 1;
                    self.build_level(time)
                } else {
                    if fire_input {
                        self.world.player_attack(time);
                    }
                    self.world.player_set_speed(lr_input);
                    self.world.tick(&mut self.led_string, time);
                    self.world.collide();
                    self.led_string.clear();
                    self.world.draw(&mut self.led_string, time);
                }
                if lr_input == 0 && !fire_input {self.input_idle_time += 1;}
                else {self.input_idle_time = 0;}
                if self.input_idle_time >= (GAME_FPS * GAME_TIMEOUT) {State::Screensaver}
                else {State::Playing}
            },
            State::Death => {
                // Reset level and decrement lives here
                // if lives 0 return State::Starting
                State::Lives
            },
            State::Lives => {
                // Render lives
                State::Playing
            },
            State::Win => {
                // Render winning animation here
                State::Starting{stage: StartStage::Wipeup, start_time: time}
            }
        };

    }

    pub fn get_led(&mut self, i: usize) -> [u8; 3] {
        let led = self.led_string.get_raw(i as i32);
        [led.r, led.g, led.b]
    }

    pub fn get_led_len(&mut self) -> usize {
        self.led_string.len() as usize
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
            5 => { // Conveyor
                self.world.spawn_conveyor(100, 600, -6);
                self.world.spawn_enemy(800, 0, 0);
            },
            6 => { // Drainage
                self.world.spawn_conveyor(100, 600, 1);
                self.world.spawn_conveyor(600, 999, -1);
                self.world.spawn_enemy(600, 0, 0);
                self.world.spawn_spawner(time, 999, 5500, -3, 0);
            },
            7 => { // Enemy swarm
                self.world.spawn_enemy(700, 3, 275);
                self.world.spawn_enemy(500, 2, 250);
                self.world.spawn_enemy(600, 3, 200);
                self.world.spawn_enemy(800, 2, 350);
                self.world.spawn_enemy(400, 3, 150);
                self.world.spawn_enemy(450, 2, 400);
            },
            8 => { // Sin enemy #2 practice (slow conveyor)
                self.world.spawn_enemy(700, 7, 275);
                self.world.spawn_enemy(500, 5, 250);
                self.world.spawn_spawner(time, 999, 5500, -4, 3000);
                self.world.spawn_spawner(time, 0, 5500, 5, 10000);
                self.world.spawn_conveyor(100, 900, -4);
            },
            9 => { // Conveyor of enemies
                self.world.spawn_conveyor(50, 998, 6);
                self.world.spawn_enemy(300, 0, 0);
                self.world.spawn_enemy(400, 0, 0);
                self.world.spawn_enemy(500, 0, 0);
                self.world.spawn_enemy(600, 0, 0);
                self.world.spawn_enemy(700, 0, 0);
                self.world.spawn_enemy(800, 0, 0);
                self.world.spawn_enemy(900, 0, 0);
            },
            10 => { // Lava run
                self.world.spawn_lava(time, 195, 300, 2000, 2000, 0, false);
                self.world.spawn_lava(time, 400, 500, 2000, 2000, 0, false);
                self.world.spawn_lava(time, 600, 700, 2000, 2000, 0, false);
                self.world.spawn_spawner(time, 999, 3800, 4, 0);
            },
            11 => { // Sin enemy #2 (fast conveyor)
                self.world.spawn_enemy(800, -7, 275);
                self.world.spawn_enemy(700, -7, 275);
                self.world.spawn_enemy(500, -5, 250);
                self.world.spawn_spawner(time, 999, 3000, -4, 3000);
                self.world.spawn_spawner(time, 0, 5500, 5, 10000);
                self.world.spawn_conveyor(100, 900, 6);
            },
            12 => { // less lava, more enemies
                self.world.spawn_lava(time, 350, 455, 2000, 2000, 0, false);
                self.world.spawn_lava(time, 660, 760, 2000, 2000, 0, false);
                self.world.spawn_spawner(time, 999, 3800, -4, 270);
                self.world.spawn_enemy(800, 0, 0);
            },
            13 => { // pushed towards lava
                self.world.spawn_conveyor(100, 800, 1);
                self.world.spawn_lava(time, 800, 850, 1000, 2000, 0, false);
                self.world.spawn_spawner(time, 999, 2000, -4, 0);
            },
            14 => { // quick lava
                self.world.spawn_spawner(time, 0, 2300, 6, 7000);
                self.world.spawn_lava(time, 200, 400, 1000, 2000, 0, false);
                self.world.spawn_lava(time, 600, 800, 1000, 2000, 0, false);
                self.world.spawn_spawner(time,999, 2500, -6, 1000);
            },
            15 => { // spawn train;
                self.world.spawn_spawner(time, 900, 1300, -2, 0);
            },
            16 => { // spawn train skinny attack width;
                self.world.player_set_attack_width(32);
                self.world.spawn_spawner(time, 900, 1800, -2, 0);
            },
            17 => { // evil fast split spawner
                self.world.spawn_spawner(time, 550, 1500, -2, 0);
                self.world.spawn_spawner(time, 550, 1500, 2, 0);
            },
            18 => { // split spawner with exit blocking lava
                self.world.spawn_spawner(time, 500, 1200, -2, 0);
                self.world.spawn_spawner(time, 500, 1200, 2, 0);
                self.world.spawn_lava(time, 900, 950, 2200, 800, 2000, false);
            },
            _ => {
                self.level = 0;
                self.build_level(time);
            }
        }
    }
}
