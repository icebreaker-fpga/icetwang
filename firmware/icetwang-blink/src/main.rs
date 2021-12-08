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

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use icetwang_pac;
use ledstr::LEDString;
use riscv_rt::entry;

mod timer;
mod rgbled;
mod print;
mod ledstr;
mod joy;
mod i2c;
mod imu;

use timer::Timer;
use rgbled::RGBLed;
use joy::Joy;

use crate::i2c::I2c;
use icm20948::{ICM20948_CHIP_ADR, ICMI2C};

//const SYSTEM_CLOCK_FREQUENCY: u32 = 21_000_000;

// This is the entry point for the application.
// It is not allowed to return.

fn real_main() -> ! {
    let peripherals = icetwang_pac::Peripherals::take().unwrap();

    // Configure uart for the print macro
    print::print_hardware::set_hardware(peripherals.UART);
    print::print_hardware::set_divider(22); // Set baud to 1MBaud

    // Configure the timer
    let mut timer = Timer::new(peripherals.TIMER);

    // Configure the RGBLed
    let mut rgbled = RGBLed::new(peripherals.RGBLED);
    rgbled.color(96, 10, 5);
    rgbled.blink(true, 200, 1000);
    rgbled.breathe(true, 100, 200);
    rgbled.state(true);

    // Configure the LED String
    let mut ledstring = LEDString::new(peripherals.LEDSTR);

    ledstring.set_len(3);
    ledstring.set_div(0);
    ledstring.write_rgb(0, ledstr::LED::new(0xFF, 0x00, 0x00));
    ledstring.write_rgb(1, ledstr::LED::new(0x00, 0xFF, 0x00));
    ledstring.write_rgb(2, ledstr::LED::new(0x00, 0x00, 0xFF));
    ledstring.write_rgb(3, ledstr::LED::new(0x44, 0x44, 0x44));
    // Output the inital LED string state
    ledstring.start();

    // Configure the Joystick
    let mut joy = Joy::new(peripherals.JOY);

    // I2C
    let mut i2c = I2c::new(peripherals.I2C);

    let mut icm = ICMI2C::<_, _, ICM20948_CHIP_ADR>::new(&mut i2c).unwrap();
    icm.init(&mut i2c, &mut timer).unwrap();
    icm.set_lp_filter(&mut i2c, &mut timer).unwrap();
    // // Print header
    println!("\n\rDir  XAcc     YAcc     ZAcc     XGyro     YGyro     ZGyro     CPU  us");

    // // Start timer
    let event_time = 16666;
    timer.reset();
    timer.set_load(event_time); // We want the timer ev to trigger every 1/60th of a second
    timer.set_reload(event_time);
    timer.enable_ev();
    timer.enable();

    let mut val: u8 = 0xFF;
    loop {
        let joystate = joy.get();
        print!("\r{}{}{}{}",
            if joystate.left {"<"} else {" "},
            if joystate.right {">"} else {" "},
            if joystate.up {"^"} else {" "},
            if joystate.down {"v"} else {" "});

        // Read and display IMU data
        let bits = icm.get_values_accel_gyro(&mut i2c).unwrap();
        let (xa, ya, za, xg, yg, zg) = icm.scale_raw_accel_gyro(bits);
        print!(" {:+08.4} {:+08.4} {:+08.4} {:+09.4} {:+09.4} {:+09.4}", xa, ya, za, xg, yg, zg);

        // Make sure the LED string is ready for us
        while ledstring.bsy_n() {
            print!("b");
        }

        val = val.wrapping_sub(1);
        ledstring.write_rgb(0, ledstr::LED::new(val, 0x00, 0x00));
        ledstring.write_rgb(1, ledstr::LED::new(0x00, 0xFF - val, 0x00));
        ledstring.write_rgb(2, ledstr::LED::new(0x00, 0x00, val));
        ledstring.start();

        let time_elapsed = event_time - timer.get_value();
        let busy_percent = (time_elapsed * 100) / event_time;
        print!(" {:03}% {}", busy_percent, time_elapsed);
        // Wait for the timer to expire
        while !timer.get_ev() {
            //println!("tmr: {:#010X} {:#06b}", timer.value(), (timer.csr() & 0xFF) as u8);
        }
        timer.ev_rst(); // Reset event
    }
}

#[entry]
fn main() -> ! {
    real_main();
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\r\n==> {}", info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
