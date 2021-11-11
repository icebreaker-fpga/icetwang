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

extern crate piston_window;
extern crate sdl2_window;
extern crate find_folder;

use std::io::{Write, stdout};

use piston_window::*;
use sdl2_window::Sdl2Window;

mod twang;
use twang::Twang;

const LED_SIZE: u32 = 12;
const LED_MARGIN: u32 = 1;
const LED_STRING_LENGTH: usize = 144;
const LED_STRING_STATUS: u32 = 13;

fn main() {

    // Create a window for our simulated LEDs
	let window_dimensions = [((LED_SIZE + LED_MARGIN) * LED_STRING_LENGTH as u32) + LED_MARGIN, (LED_SIZE + (LED_MARGIN * 2)) + LED_STRING_STATUS + LED_MARGIN];
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Rusty Spring aka rTWANG!", Size::from(window_dimensions))
        .exit_on_esc(true)
        .resizable(false)
        //.graphics_api(OpenGL::V3_2)
        .fullscreen(false)
        .build()
        .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("terminal-grotesque.ttf");
    let mut glyphs = window.load_font(font).unwrap();

    // Try to get as close as possible to 60fps
    window.set_ups(60);

    println!("dim {:?}", window_dimensions);

    // Game objects
    let mut twang = Twang::new();

    // Game loop
    let mut red: u8 = 100;
    let mut frames = 0;
    let mut passed = 0.0;
    let mut ftime = 0.0;
    let mut time: u32; // time in msec
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut lr_input: i32 = 0;
    let mut fps = 0.0;
    let mut status = format!("Heya!");
    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, device| {
                clear([0.33; 4], graphics);
                let len = twang.get_raw_led_len();
                for i in 0..len {
                    let led = twang.get_raw_led(i);
                    // convert to f32 and apply inverse gamma to match LEDs
                    let r = (led[0] as f32 / 255.0).powf(1.0/2.2);
                    let g = (led[1] as f32 / 255.0).powf(1.0/2.2);
                    let b = (led[2] as f32 / 255.0).powf(1.0/2.2);
                    rectangle([r, g, b, 1.0],
                              [1.0 + ((LED_SIZE + LED_MARGIN) * (i as u32)) as f64, LED_MARGIN as f64, LED_SIZE as f64, LED_SIZE as f64],
	                          context.transform,
	                          graphics);
	           }
               let transform = context.transform.trans(1.0, 25.0);
               status = format!("FPS: {:.2} DIR: {}{}{}", fps, if left {"<"} else {" "}, if right {">"} else {" "}, if up {"^"} else {" "});
               text::Text::new_color([1.0, 1.0, 1.0, 1.0], 10).draw(
                &status.to_string(),
                &mut glyphs,
                &context.draw_state,
                transform,
                graphics).unwrap();

               // Update glyphs before rendering.
               glyphs.factory.encoder.flush(device);
            });
            frames += 1;
        }

        // Keyboard inputs
        if let Some(button) = event.press_args() {
            if button == Button::Keyboard(Key::Left) {
                lr_input -= 10;
                left = true;
            }
            if button == Button::Keyboard(Key::Right) {
                lr_input += 10;
                right = true;
            }
            if button == Button::Keyboard(Key::Up) {
                up = true;
            }
        }

        if let Some(button) = event.release_args() {
            if button == Button::Keyboard(Key::Left) {
                lr_input += 10;
                left = false;
            }
            if button == Button::Keyboard(Key::Right) {
                lr_input -= 10;
                right = false;
            }
            if button == Button::Keyboard(Key::Up) {
                up = false;
            }
        }

        // Game update & FPS counter
        if let Some(u) = event.update_args() {
            red = red.wrapping_add(1);

            passed += u.dt;
            ftime += u.dt;
            time = (ftime * 1_000.0).round() as u32;

            if passed > 0.01 {
                fps = (frames as f64) / passed;
                status = format!("FPS: {:.2} TIM: {}", fps, time);
                print!("FPS: {:.2} TIM: {:010} DIR: {}{}{}\r", fps, time, if left {"<"} else {" "}, if right {">"} else {" "}, if up {"^"} else {" "});
                let _ = stdout().flush();
                frames = 0;
                passed = 0.0;
            }

            twang.cycle(lr_input, up, time);
        }
    }
}
