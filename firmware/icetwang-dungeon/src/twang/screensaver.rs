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

use super::led_string::*;

/*
const DOTSPEED: u32 = 11;
const DOTS_IN_BOWLS_COUNT: u32 = 3;
const DOT_DISTANCE: u32 = 65535 / DOTS_IN_BOWLS_COUNT as u32;
const DOT_BRIGHTNESS: u8 = 255;
*/

pub fn tick(led_string: &mut LEDString, time: u32) {
    let mode = (time / 3000) % 5;
    //let mode = 4;

    match mode {
        /*0 => {
            // Marching green <> orange
            led_string.nscale8(250);

            let n = ((time / 250) % 10) as usize;
            let c = ((20.0 + libm::sinhf(((time as f32 / 5000.0).to_radians()) * 255.0 + 1.0) * 33.0) % 256.0) as u8;
            for i in 0..led_string.len() {
                if (i % 10) == n {
                    led_string[i].set_hsv(c, 255, 150);
                }
            }
        }*/
        0|1 => {
            // Random flashes
            led_string.nscale8(250);

            for i in 0..led_string.len() {
                let val = time + i as u32;
                if (val ^ (val >> 1)) < 0x000000FF {
                    led_string[i].set_hsv(25, 255, 100)
                }
            }
        }
        /*2 => {
            // Dots in bowl
            led_string.clear();

            for i in 0..DOTS_IN_BOWLS_COUNT {
                let mm = (i * DOT_DISTANCE) + time.wrapping_mul(DOTSPEED);
                let mm16 = mm % (1 << 16); // Trim to 16bit
                let mmf = (mm16 as f32) / libm::powf(2.0, 15.0); // map to 0.0 - 2.0 range
                let nsin = (libm::sinhf(mmf * core::f32::consts::PI) + 1.0) / 2.0;
                let n = ((led_string.len() as f32 - 5.0) * nsin + 2.0) as usize;
                let c: u8 = (mm / 50 % 255) as u8;
                // println!("i {} mm {:#010X} mm16 {:#06X} mmf {:.4} nsin {:2.4}, n {:03}, c {:#04X}", i, mm, mm16, mmf, nsin, n, c);
                led_string[n - 2] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 4));
                led_string[n - 1] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 2));
                led_string[n + 0] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS));
                led_string[n + 1] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 2));
                led_string[n + 2] += LED::new(hsv_rainbow(c, 255, DOT_BRIGHTNESS / 4));
            }
        }*/
        2 | 3 => {
            // Sparkles
            led_string.nscale8(128);

            let c = time % 800;
            let n;
            if c < 240 {
                n = 121 - c / 2;
            } else {
                n = 1;
            }

            for i in 0..led_string.len() {
                let x = time + i as u32;
                let val: u8 = ((x ^ (x >> 1)) & 0xFF) as u8; // should be rand
                if val <= (n as u8) {
                    led_string[i].set_rgb([100; 3]);
                }
            }
        }
        4 => {
            // Scroll dots
            for i in 0..led_string.len() {
                if (i + (time as usize / 100)) % 5 == 0 {
                    led_string[i].set_rgb([100; 3]);
                } else {
                    led_string[i].set_rgb([0; 3]);
                }
            }
        }
        _ => ()
    }
}