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

use core::ops::{Index, IndexMut, AddAssign};

use crate::LED_STRING_LENGTH;
use crate::twang::utils::range_map;
// use std::iter::IntoIterator;
const LED_STRING_VLENGTH: usize = 1000;

/*****************************************************************************
 * LED
 *****************************************************************************/

#[derive(Clone, Debug, Copy)]
pub struct LED {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LED {
    pub fn new(color: [u8; 3]) -> LED {
        LED {
            r: color[0],
            g: color[1],
            b: color[2]
        }
    }

    pub fn set_rgb(&mut self, rgb: [u8; 3]) {
        self.r = rgb[0];
        self.g = rgb[1];
        self.b = rgb[2];
    }

    pub fn set_hsv(&mut self, h: u8, s: u8, v: u8) {
        self.set_rgb(hsv_rainbow(h, s, v));
    }

    pub fn nscale8(&mut self, scale: u8) {
        self.r = scale8(self.r, scale);
        self.g = scale8(self.g, scale);
        self.b = scale8(self.b, scale);
    }
}

impl AddAssign for LED {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r.saturating_add(rhs.r);
        self.g = self.g.saturating_add(rhs.g);
        self.b = self.b.saturating_add(rhs.b);
    }
}

/*****************************************************************************
 * LEDString
 *****************************************************************************/

#[derive(Debug)]
pub struct LEDString {
	leds: [LED; LED_STRING_LENGTH],
    null: LED,
}

impl LEDString {

	pub fn new() -> LEDString {
		LEDString {
			leds: [LED::new([0; 3]); LED_STRING_LENGTH],
            null: LED::new([0; 3])
		}
	}

    pub fn len(&self) -> usize {
        LED_STRING_VLENGTH
    }

    pub fn raw_len(&self) -> usize {
        self.leds.len()
    }

    pub fn clear(&mut self){
        for led in &mut self.leds {
            led.set_rgb([0; 3]);
        }
    }

    pub fn nscale8(&mut self, scale: u8) {
        for led in &mut self.leds {
            led.nscale8(scale);
        }
    }

    pub fn get_raw(&mut self, i: usize) -> &LED {
        if i >= self.leds.len() {
            &self.null
        } else {
            &self.leds[i]
        }
    }
}

impl Index<usize> for LEDString {
    type Output = LED;

    fn index(&self, i: usize) -> &Self::Output {
        let ri = range_map(i, 0, LED_STRING_VLENGTH, 0, LED_STRING_LENGTH);

        if ri >= self.leds.len() {
            &self.null
        } else {
            &self.leds[ri]
        }
    }
}

impl IndexMut<usize> for LEDString {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let ri = range_map(i, 0, LED_STRING_VLENGTH, 0, LED_STRING_LENGTH);

        if ri >= self.leds.len() {
            &mut self.null
        } else {
            &mut self.leds[ri]
        }
    }
}

/*****************************************************************************
 * Value operator functions
 *****************************************************************************/

// scale one byte by a second one, which is treated as the numerator of a fraction whose denominator is 256
// In other words, it computes i * (scale / 256)
 pub fn scale8(val: u8, scaler: u8) -> u8 {
    let val = val as u32;
    let scaler = scaler as u32;
    ((val * scaler) / 255) as u8
 }

// The "video" version of scale8 guarantees that the output will be only be zero if one or both of the inputs are zero.
// If both inputs are non-zero, the output is guaranteed to be non-zero. This makes for better 'video'/LED dimming, at
// the cost of several additional cycles.
pub fn scale8_video(val: u8, scaler: u8) -> u8 {
    let mut ret = (val as u16 * scaler as u16) >> 8 as u8;
    if val > 0 && scaler > 0 {
        ret += 1;
    }

    ret as u8
}

 pub fn hsv_rainbow(h: u8, s: u8, v: u8) -> [u8; 3] {
    // Yellow has a higher inherent brightness than any other color; pure yellow is perceived to be 93% as bright
    // as white. In order to make yellow appear the correct relative brightness, it has to be rendered brighter
    // than all other colors.
    // Level y1 is a moderate boost, the default.
    // Level y2 is a strong boost.
    let y1 = true;
    let y2 = false;

    // g2: Whether to divide all greens by two.
    // Depends greatly on your particular LEDs
    let g2 = false;

    // g_scale: what to scale green down by.
    // Depends GREATLY on your particular LEDs
    let g_scale = 0;

    let offset = h & 0x1F;

    let offset8 = offset << 3;

    let third = scale8(offset8, 255 / 3);  // max = 85
    let twothirds: u16 = (256 * 2) / 3;
    let twothirds = scale8(offset8, twothirds as u8);

    let (mut r, mut g, mut b) = (0, 0, 0);
    let section = (h & 0xE0) >> 5;
    match section {
    	0 => {
    		// 000
        	// case 0: R -> O
        	r = 255 - third;
        	g = third;
        	b = 0;
        }
        1 => {
	        // 001
	        // case 1: O -> Y
	        if y1 {
	            r = 171;
	            g = 85 + third;
	            b = 0;
            }
	        if y2 {
	            r = 170 + third;
	            g = 85 + twothirds;
	            b = 0;
            }
        },
        2 => {
            // 010
            // case 2: Y -> G
            if y1 {
                r = 171 - twothirds;
                g = 170 + third;
                b = 0;
            }
            if y2 {
                r = 255 - offset8;
                g = 255;
                b = 0;
            }
        },
        3 => {
            // 011
            // case 3: G -> A
            r = 0;
            g = 255 - third;
            b = third;
        },
        4 => {
            // 100
            // case 4: A -> B
            r = 0;
            g = 171 - twothirds;  // 170?
            b = 85 + twothirds;
        },
        5 => {
            // 101
            // case 5: B -> P
            r = third;
            g = 0;
            b = 255 - third;
        },
        6 => {
            // 110
            // case 6: P - - K
            r = 85 + third;
            g = 0;
            b = 171 - third;
        },
        7 => {
            // 111
            // case 7: # K -> R
            r = 170 + third;
            g = 0;
            b = 85 - third;
        },
        _ => ()
    }

    // println!("sec: {}", section);

    // This is one of the good places to scale the green down,
    // although the client can scale green down as well.
    if g2 {
        g = g >> 1;
    }
    if g_scale > 0 {
        g = scale8_video(g, g_scale);
    }

    // Scale down colors if we're desaturated at all
    // and add the brightness_floor to r, g, and b.
    if s != 255 {
        if s == 0 {
            r = 255;
            g = 255;
            b = 255;
        } else {
            // nscale8x3_video( r, g, b, sat)
            if r > 0 {
                r = scale8(r, s);
            }
            if g > 0 {
                g = scale8(g, s);
            }
            if b > 0 {
                b = scale8(b, s);
            }

            let mut desat = 255 - s;
            desat = scale8(desat, desat);

            let brightness_floor = desat;
            r += brightness_floor;
            g += brightness_floor;
            b += brightness_floor;
        }
    }

    // Now scale everything down if we're at value < 255.
    if v != 255 {
        let v = scale8_video(v, v);
        if v == 0 {
            r = 0;
            g = 0;
            b = 0;
        } else {
            // nscale8x3_video( r, g, b, val)
            if r > 0 {
                r = scale8(r, v)
            }
            if g > 0 {
                g = scale8(g, v)
            }
            if b > 0 {
                b = scale8(b, v)
            }
        }
    }

    [r, g, b]
}