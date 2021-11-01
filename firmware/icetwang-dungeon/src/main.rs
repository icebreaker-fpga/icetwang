#![no_std]
#![no_main]

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

//const SYSTEM_CLOCK_FREQUENCY: u32 = 24_000_000;
const LED_DEFAULT_COLOR: [u8; 3] = [0; 3];
const LED_STRING_LENGTH: u16 = 144;

// This is the entry point for the application.
// It is not allowed to return.

fn real_main() -> ! {
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

    ledstring_hal.set_len(LED_STRING_LENGTH - 1); // The HAL min length is 1 represented by 0
    ledstring_hal.set_div(0);
    ledstring_hal.set_glob(1);
    for i  in 0..LED_STRING_LENGTH {
        ledstring_hal.write_rgb(i, [i as u8, 0x00, 0x00]);
    }
    // Output the inital LED string state
    ledstring_hal.start();

    // Configure the Joystick
    let mut joy = Joy::new(peripherals.JOY);

    // Print header
    println!("\nDir  CPU  us");

    // Start timer
    timer.enable();

    let mut val: u8 = 0xFF;
    loop {
        let joystate = joy.get();
        print!("{}{}{}{}",
            if joystate.left {"<"} else {" "},
            if joystate.right {">"} else {" "},
            if joystate.up {"^"} else {" "},
            if joystate.down {"v"} else {" "});

        // Make sure the LED string is ready for us
        let mut bsy = false;
        while ledstring_hal.bsy_n() {
            print!("b");
            bsy = true;
        }
        if bsy {
            println!("");
        }

        val = val.wrapping_sub(1);
        ledstring_hal.write_rgb(0, [val,  0x00,       0x00]);
        ledstring_hal.write_rgb(1, [0x00, 0xFF - val, 0x00]);
        ledstring_hal.write_rgb(2, [0x00, 0x00,       val]);
        ledstring_hal.start();

        let time_elapsed = event_time - timer.value();
        let busy_percent = (time_elapsed * 100) / event_time;
        print!(" {:03}% {}\r", busy_percent, time_elapsed);
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

