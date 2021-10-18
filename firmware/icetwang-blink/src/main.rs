#![no_std]
#![no_main]

extern crate panic_halt;

use icetwang_pac;
use riscv_rt::entry;

//mod timer;
mod leds;
mod print;

//use timer::Timer;
use leds::Leds;

//const SYSTEM_CLOCK_FREQUENCY: u32 = 21_000_000;

// This is the entry point for the application.
// It is not allowed to return.

fn real_main() -> ! {
    let peripherals = icetwang_pac::Peripherals::take().unwrap();

    print::print_hardware::set_hardware(peripherals.UART);
    print::print_hardware::set_divider(22); // Set baud to 1MBaud
    //let mut timer = Timer::new(peripherals.TIMER0);
    let mut leds = Leds::new(peripherals.LED_COMMON);
    leds.color(96, 10, 5);
    leds.blink(true, 200, 1000);
    leds.breathe(true, 100, 200);
    leds.state(true);

    loop {
        print!("a");
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
