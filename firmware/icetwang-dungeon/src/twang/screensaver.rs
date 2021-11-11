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

use super::led_string::{LEDString, LED, hsv_rainbow};
use super::utils::sinu8;
use crate::print;

const DOTSPEED: u32 = 11;
const DOTS_IN_BOWLS_COUNT: u32 = 3;
const DOT_DISTANCE: u32 = 65535 / DOTS_IN_BOWLS_COUNT as u32;
const DOT_BRIGHTNESS: u8 = 255;

struct PRand {
    seed: u32
}

impl PRand {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        let a: u32 = 134775813;
        let c: u32 = 1;
        self.seed = a.wrapping_mul(self.seed).wrapping_add(c);
        self.seed
    }
}

pub struct Screensaver {
    rand: PRand
}

impl Screensaver {
    pub fn new(seed: u32) -> Self {
        Self { rand: PRand::new(seed) }
    }

    pub fn tick(&mut self, led_string: &mut LEDString, time: u32) {
        let mode = (time / 5000) % 6;
        print!("Mode {} ", mode);
        //let mode = 3;

        match mode {
            0 => {
                // Marching green <> orange
                led_string.nscale8(250);

                let n = ((time / 250) % 100) as usize;
                let c = 20 + (((sinu8(((time / 62) & 0xFF) as u8) as u32 * 66) / 255) % 255) as u8;
                for i in (n..led_string.len()).step_by(100) {
                    led_string[i].set_hsv(c, 255, 150);
                }
            },
            1 => {
                // Random flashes
                led_string.nscale8(250);

                for i in (0..led_string.len()).step_by(5) {
                    if (self.rand.next_u32() % 1000) == 0 {
                        led_string[i].set_hsv(25, 255, 100)
                    }
                }
            },
            2 | 3 => {
                // Dots in bowl
                led_string.clear();

                for i in 0..DOTS_IN_BOWLS_COUNT {
                    let mm = (i * DOT_DISTANCE) + time.wrapping_mul(DOTSPEED);
                    let mm16 = mm % (1 << 16); // Trim to 16bit
                    let mmf = (mm16 >> 8) & 0xFF; // map to 0 - 255 range
                    let nsin = sinu8(mmf as u8) as u32;
                    let n = 2 + ((led_string.len() as u32 - 5).wrapping_mul(nsin) / 255) as usize;
                    let c: u8 = (mm / 50 % 255) as u8;
                    // println!("i {} mm {:#010X} mm16 {:#06X} mmf {:.4} nsin {:2.4}, n {:03}, c {:#04X}", i, mm, mm16, mmf, nsin, n, c);
                    led_string[n - 2] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 4));
                    led_string[n - 1] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 2));
                    led_string[n + 0] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS));
                    led_string[n + 1] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 2));
                    led_string[n + 2] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 4));
                }
            },
            4 => {
                // Sparkles
                led_string.nscale8(128);

                let c = time % 800;
                let n;
                if c < 240 {
                    n = 121 - c / 2;
                } else {
                    n = 1;
                }

                for i in (0..led_string.len()).step_by(6) {
                    let val: u8 = (self.rand.next_u32() & 0xFF) as u8;
                    if val <= (n as u8) {
                        led_string[i].set_rgb([100; 3]);
                    }
                }
            },
            5 => {
                // Scroll dots
                for i in (0..led_string.len()).step_by(6) {
                    if ((i/6) + (time as usize / 100)) % 5 == 0 {
                        led_string[i].set_rgb([100; 3]);
                    } else {
                        led_string[i].set_rgb([0; 3]);
                    }
                }
            },
            _ => ()
        }
    }
}
