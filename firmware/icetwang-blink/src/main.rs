#![no_std]
#![no_main]

extern crate panic_halt;

use icetwang_pac;
use ledstr::LEDString;
use riscv_rt::entry;

mod timer;
mod rgbled;
mod print;
mod ledstr;

use timer::Timer;
use rgbled::RGBLed;

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
    timer.load(16666); // We want the timer ev to trigger every 1/60th of a second
    timer.reload(16666);
    timer.enable_ev();

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

    // Start timer
    timer.enable();

    // let mut div: u32 = 0x00;
    let mut val: u8 = 0xFF;
    loop {
        print!("a");
        while ledstring.bsy_n() {
            print!("b");
        }

        val = val.wrapping_sub(1);
        ledstring.write_rgb(0, ledstr::LED::new(val, 0x00, 0x00));
        ledstring.write_rgb(1, ledstr::LED::new(0x00, 0xFF - val, 0x00));
        ledstring.write_rgb(2, ledstr::LED::new(0x00, 0x00, val));
        ledstring.start();

        // Wait for the timer to expire
        while !timer.ev_n() {
            //println!("tmr: {:#010X} {:#06b}", timer.value(), (timer.csr() & 0xFF) as u8);
        }
        timer.ev_rst(); // Reset event
    }
}

#[entry]
fn main() -> ! {
    real_main();
}

