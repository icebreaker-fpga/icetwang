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
mod led_string;
mod twang;

//const SYSTEM_CLOCK_FREQUENCY: u32 = 24_000_000;
const LED_DEFAULT_COLOR: [u8; 3] = [0; 3];
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
    let mut led_string = led_string::LEDString::new(LED_DEFAULT_COLOR);
    let mut twang = twang::Twang::new();
    let mut lr_input: i32;
    let mut fire_input: bool;
    let mut time: u32 = 0;

    // Print debug header
    println!("\nDir  CPU  us");

    // Start timer
    timer.enable();

    //let mut val: u8 = 0xFF;
    loop {
        let joystate = joy.get();
        print!("{}{}{}{}",
            if joystate.left {"<"} else {" "},
            if joystate.right {">"} else {" "},
            if joystate.up {"^"} else {" "},
            if joystate.down {"v"} else {" "});

        // Cycle game logic
        lr_input = 0;
        if joystate.left {
            lr_input = -1;
        }
        if joystate.right {
            lr_input =  1;
        }
        fire_input = joystate.up || joystate.down;

        twang.cycle(lr_input, fire_input, &mut led_string, time);

        // Make sure the LED string is ready for us
        let mut bsy = false;
        while ledstring_hal.bsy_n() {
            print!("b");
            bsy = true;
        }
        if bsy {
            println!("");
        }

        for i in 0..LED_STRING_LENGTH as u16 {
            let led = &led_string[i as usize];
            ledstring_hal.write_rgb(i, [led.r, led.g, led.b]);
        }
        ledstring_hal.start();

        let time_elapsed = event_time - timer.value();
        let busy_percent = (time_elapsed * 100) / event_time;
        print!(" {:03}% {} {}\r", busy_percent, time_elapsed, time);
        // Wait for the timer to expire
        while !timer.ev_n() {
            //println!("tmr: {:#010X} {:#06b}", timer.value(), (timer.csr() & 0xFF) as u8);
        }
        timer.ev_rst(); // Reset event

        // Advance time
        time = time.wrapping_add(event_time/1000);
    }
}

#[entry]
fn main() -> ! {
    real_main();
}

