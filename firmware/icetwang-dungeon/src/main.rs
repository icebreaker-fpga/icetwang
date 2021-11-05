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

#![no_std]
#![no_main]

// Hardware crates
extern crate panic_halt;

use icetwang_pac;
use ledstr_hal::LEDStringHAL;
use riscv_rt::entry;

mod timer;
mod rgbled;
mod print;
mod ledstr_hal;
mod joy;

use timer::Timer;
use rgbled::RGBLed;
use joy::Joy;

// Game crates
mod twang;

const LED_STRING_LENGTH: usize = 144;

// This is the entry point for the application.
// It is not allowed to return.

fn real_main() -> ! {

    // Initialize hardware
    // -------------------
    let peripherals = icetwang_pac::Peripherals::take().unwrap();

    // Configure uart for the print macro
    print::print_hardware::set_hardware(peripherals.UART);
    print::print_hardware::set_divider(22); // Set baud to 1MBaud

    // Configure the timer
    let mut timer = Timer::new(peripherals.TIMER);
    let event_time = 16000;
    timer.load(event_time); // We want the timer ev to trigger every 1/60th of a second
    timer.reload(event_time);
    timer.enable_ev();

    // Configure the RGBLed
    let mut rgbled = RGBLed::new(peripherals.RGBLED);
    rgbled.color(96, 10, 5);
    rgbled.blink(true, 200, 1000);
    rgbled.breathe(true, 100, 200);
    rgbled.state(true);

    // Configure the LED String
    let mut ledstring_hal = LEDStringHAL::new(peripherals.LEDSTR);

    ledstring_hal.set_len(LED_STRING_LENGTH as u16 - 1); // The HAL min length is 1 represented by 0
    ledstring_hal.set_div(0);
    ledstring_hal.set_glob(1);
    for i  in 0..LED_STRING_LENGTH as u16 {
        ledstring_hal.write_rgb(i, [i as u8, 0x00, 0x00]);
    }
    // Output the inital LED string state
    ledstring_hal.start();

    // Configure the Joystick
    let mut joy = Joy::new(peripherals.JOY);

    // Initialize Game
    // ---------------
    let mut twang = twang::Twang::new();
    let mut lr_input: i32;
    let mut fire_input: bool;
    let mut time: u32 = 0;

    // Print debug header
    println!("\nDir  CPU  us");

    // Start timer
    timer.enable();

    // Main system loop
    loop {
        // Get joystick input
        let joystate = joy.get();
        print!("{}{}{}{}",
            if joystate.left {"<"} else {" "},
            if joystate.right {">"} else {" "},
            if joystate.up {"^"} else {" "},
            if joystate.down {"v"} else {" "});

        // Cycle game logic
        lr_input = 0;
        if joystate.left {
            lr_input = -10;
        }
        if joystate.right {
            lr_input =  10;
        }
        fire_input = joystate.up || joystate.down;

        twang.cycle(lr_input, fire_input, time);

        // Make sure the LED string is ready for us
        let mut bsy = false;
        while ledstring_hal.bsy_n() {
            print!("b");
            bsy = true;
        }
        if bsy {
            println!(""); // Add a newline if we printed some delay indicators
        }

        // Send the LED values to the hardware
        let len = twang.get_raw_led_len();
        for i in 0..len {
            let led = twang.get_raw_led(i as usize);
            ledstring_hal.write_rgb((len - 1 - i) as u16, led);
        }
        ledstring_hal.start();

        // Calculate elapsed and percentage of the frame time
        let time_elapsed = event_time - timer.value();
        let busy_percent = (time_elapsed * 100) / event_time;
        print!(" {:03}% {:5} {:10}\r", busy_percent, time_elapsed, time);

        // Wait for the timer to expire
        while !timer.ev_n() {
            //println!("tmr: {:#010X} {:#06b}", timer.value(), (timer.csr() & 0xFF) as u8);
        }
        timer.ev_rst(); // Reset event

        // Advance time (we track time in msec but the timer runs in usec)
        time = time.wrapping_add(event_time/1000);
    }
}

#[entry]
fn main() -> ! {
    real_main();
}

