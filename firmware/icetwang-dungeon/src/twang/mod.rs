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
mod attract;
mod world;
mod player;
mod enemy;
mod spawner;
mod lava;
mod conveyor;
mod rand;
mod particle;
mod boss;

use world::World;
use led_string::LEDString;

use self::rand::random8lim;

#[cfg(feature = "icetwanghw")]
use super::print;
use utils::{range_map, constrain};

const GAME_TIMEOUT: u32 = 60000;
const STARTUP_WIPEUP_DUR: u32 = 200;
const STARTUP_SPARKLE_DUR: u32 = 1300;
const STARTUP_FADE_DUR: u32 = 1500;
const DEATH_EXPLOSION_DUR: u32 = 200;
const DEATH_EXPLOSION_WIDTH: i32 = 10;
const LIVES_DISPLAY_DUR: u32 = 1000;
const PLAYER_DEFAULT_LIVES: u8 = 3;
const GAMEOVER_SPREAD_DUR: u32 = 1000;
const GAMEOVER_FADE_DUR: u32 = 1500;

#[derive(Clone, Copy)]
enum StartStage {
    Wipeup,
    Sparkle,
    Fade,
}

#[derive(Clone, Copy)]
enum DeathStage {
    Explosion,
    Particles
}

#[derive(Clone, Copy)]
enum GameOverStage {
    Spread,
    Fade
}

enum State {
    Screensaver,
    Starting{stage: StartStage, start_time: u32},
    Playing{level: u32, timeout: u32},
    Death{level: u32, stage: DeathStage, start_time: u32},
    Lives{level: u32, start_time: u32},
    GameOver{stage: GameOverStage, start_time: u32},
    Win,
}

pub struct Twang {
    led_string: LEDString,
    screensaver: attract::Attract,
    state: State,
    world: World,
}

impl Twang {
    pub fn new() -> Twang {
        Twang {
            led_string: LEDString::new(),
            screensaver: attract::Attract::new(),
            state: State::Screensaver,
            world: World::new(),
        }
    }

    pub fn cycle(&mut self, lr_input: i32, fire_input: bool, time: u32) {

        self.state = match self.state {
            State::Screensaver => {
                self.screensaver.tick(&mut self.led_string, time);
                if lr_input != 0 || fire_input {
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
                            State::Starting{stage: StartStage::Wipeup, start_time}
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
                            State::Starting{stage: StartStage::Sparkle, start_time}
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
                            State::Starting{stage: StartStage::Fade, start_time}
                        } else {
                            self.world.player_set_lives(PLAYER_DEFAULT_LIVES);
                            self.build_level(0, time);
                            State::Playing{level: 0, timeout: time}
                        }
                    }
                }
            },
            State::Playing{level, timeout} => {
                print!("LVL {} ", level);

                if fire_input {
                    self.world.player_attack(time);
                }
                self.world.player_set_speed(lr_input);
                self.world.tick(&mut self.led_string, time);
                self.world.collide(time);
                self.led_string.clear();
                self.world.draw(&mut self.led_string, time);

                // Decide state transition
                if (time - timeout) > GAME_TIMEOUT {
                    State::Screensaver
                } else if !self.world.player_alive() {
                    let pos = self.world.player_position();
                    self.world.spawn_particles(pos);
                    State::Death{level, stage: DeathStage::Explosion, start_time: time}
                } else if self.world.exit_n() {
                    self.build_level(level + 1, time);
                    State::Playing{level: level + 1, timeout: time}
                } else if lr_input == 0 && !fire_input {
                    State::Playing{level, timeout}
                } else {
                    State::Playing{level, timeout: time}
                }
            },
            State::Death {level, stage, start_time} => {
                self.led_string.clear();
                // Death Animation
                match stage {
                    DeathStage::Explosion => {
                        self.world.cycle_particles(&mut self.led_string, false, 0);
                        let brightness = range_map(time - start_time, 0, DEATH_EXPLOSION_DUR, 255, 50) as u8;
                        let pos = self.led_string.vtor(self.world.player_position());
                        let start = constrain(range_map((time - start_time) as i32, 0, DEATH_EXPLOSION_DUR as i32, pos, pos - DEATH_EXPLOSION_WIDTH), 0, self.led_string.len() - 1);
                        let stop = constrain(range_map((time - start_time) as i32, 0, DEATH_EXPLOSION_DUR as i32, pos, pos + DEATH_EXPLOSION_WIDTH), 0, self.led_string.len() - 1);
                        for i in start..stop {
                            self.led_string[i].set_rgb([255, brightness, brightness]);
                        }
                        if time < (start_time + DEATH_EXPLOSION_DUR) {
                            State::Death{level, stage: DeathStage::Explosion, start_time}
                        } else {
                            State::Death{level, stage: DeathStage::Particles, start_time: time}
                        }
                    },
                    DeathStage::Particles => {
                        if self.world.cycle_particles(&mut self.led_string, true, 0) {
                            State::Death{level, stage: DeathStage::Particles, start_time}
                        } else {
                            if level == 0 {
                                self.world.player_set_lives(PLAYER_DEFAULT_LIVES);
                            }
                            State::Lives{level, start_time: time}
                        }
                    }
                }
            },
            State::Lives{level, start_time} => {
                if self.world.player_lives() == 0 {
                    //State::Starting{stage: StartStage::Wipeup, start_time: time}
                    State::GameOver{stage: GameOverStage::Spread, start_time: time}
                } else {
                    self.led_string.clear();
                    // Render lives
                    let mut pos = 0;
                    for _ in 0..self.world.player_lives() {
                        for _ in 0..4 {
                            self.led_string[pos].set_rgb([0, 255, 0]);
                            pos += 1;
                        }
                        pos += 1;
                    }
                    if time < (start_time + LIVES_DISPLAY_DUR) {
                        State::Lives{level, start_time}
                    } else {
                        self.build_level(level, time);
                        State::Playing{level: level, timeout: time}
                    }
                }
            },
            State::GameOver{stage, start_time} => {
                self.led_string.clear();
                match stage {
                    GameOverStage::Spread => {
                        let pos = self.led_string.vtor(self.world.player_position());
                        let start = range_map((time - start_time) as i32, 0, GAMEOVER_SPREAD_DUR as i32, pos, 0);
                        let stop = range_map((time - start_time) as i32, 0, GAMEOVER_SPREAD_DUR as i32, pos, self.led_string.len() - 1);
                        //println!("t{} d{} p{} strt{} stop{} ", (time - start_time) as i32, GAMEOVER_SPREAD_DUR as i32, pos, start, stop);
                        for i in start..stop {
                            self.led_string[i].set_rgb([255, 0, 0]);
                        }
                        if time < (start_time + GAMEOVER_SPREAD_DUR) {
                            State::GameOver{stage: GameOverStage::Spread, start_time}
                        } else {
                            State::GameOver{stage: GameOverStage::Fade, start_time: time}
                        }
                    },
                    GameOverStage::Fade => {
                        let stop = range_map((time - start_time) as i32, GAMEOVER_FADE_DUR as i32, 0, 0, self.led_string.len() - 1).max(0);
                        let brightness = range_map(time - start_time, 0, GAMEOVER_FADE_DUR, 255, 0) as u8;
                        for i in 0..stop {
                            self.led_string[i].set_rgb([brightness, 0, 0]);
                        }
                        if time < (start_time + GAMEOVER_FADE_DUR) {
                            State::GameOver{stage: GameOverStage::Fade, start_time}
                        } else {
                            self.world.player_set_lives(PLAYER_DEFAULT_LIVES);
                            self.build_level(0, time);
                            State::Playing{level: 0, timeout: time}
                        }
                    }
                }
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

    fn build_level(&mut self, level: u32, time: u32) {
        self.world.reset();

        // Only level 0 starts with the player at a different position than 0
        if level == 0 {
            self.world.spawn_player(200);
        } else {
            self.world.spawn_player(0);
        }

        // Setup the rest of the level
        match level {
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
            19 => {
                self.world.spawn_boss(time, [600, 200, 800], [1000, 1600, 1800]);
            },
            _ => {
                panic!("Trying to build invalid level {}.", level);
            }
        }
    }
}
