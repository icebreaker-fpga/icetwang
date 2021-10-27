#![no_std]
#![no_main]

extern crate panic_halt;

use icetwang_pac;
use ledstr::LEDString;
use riscv_rt::entry;

//mod timer;
mod rgbled;
mod print;
mod ledstr;

//use timer::Timer;
use rgbled::RGBLed;

//const SYSTEM_CLOCK_FREQUENCY: u32 = 21_000_000;

// This is the entry point for the application.
// It is not allowed to return.

fn real_main() -> ! {
    let peripherals = icetwang_pac::Peripherals::take().unwrap();

    print::print_hardware::set_hardware(peripherals.UART);
    print::print_hardware::set_divider(22); // Set baud to 1MBaud
    //let mut timer = Timer::new(peripherals.TIMER0);
    let mut rgbled = RGBLed::new(peripherals.RGBLED);
    rgbled.color(96, 10, 5);
    rgbled.blink(true, 200, 1000);
    rgbled.breathe(true, 100, 200);
    rgbled.state(true);

    let ledstring = LEDString::new(peripherals.LEDSTR);

    let led0 = ledstring.read_rgb(0);
    println!("r{:#04x} g{:#04x} b{:#04x}", led0.r, led0.g, led0.b);
    ledstring.write_rgb(0, ledstr::LED::new(0xFF, 0xFF, 0xFF));

    let led1 = ledstring.read_rgb(0);
    println!("r{:#04x} g{:#04x} b{:#04x}", led1.r, led1.g, led1.b);

    loop {
        //print!("a");
        //leds.toggle();
        //msleep(&mut timer, 160);
    }
}

#[entry]
fn main() -> ! {
    real_main();
}

/*fn msleep(timer: &mut Timer, ms: u32) {
    timer.disable();

    timer.reload(0);
    timer.load(SYSTEM_CLOCK_FREQUENCY / 1_000 * ms);

    timer.enable();

    // Wait until the time has elapsed
    while timer.value() > 0 {}
}*/
