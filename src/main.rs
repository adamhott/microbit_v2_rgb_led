#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use microbit::hal::prelude::*;
use microbit::hal::gpio::{p0::Parts as P0Parts, Level};
use microbit::hal::pac::Peripherals;
use microbit::hal::timer::Timer;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Peripherals::take().unwrap();
    let gpio = P0Parts::new(board.P0);
    let mut timer = Timer::new(board.TIMER0);

    // Set up RGB pins
    //Pin 9 on Breakout Board
    let mut red_led = gpio.p0_09.into_push_pull_output(Level::Low);
    //Pin 3 on Breakout Board
    let mut green_led = gpio.p0_31.into_push_pull_output(Level::Low);
    //Pin 4 on Breakout Board
    let mut blue_led = gpio.p0_28.into_push_pull_output(Level::Low);

    loop {
        rprintln!("Red On!");
        red_led.set_high().unwrap();
        timer.delay_ms(250_u32);
        red_led.set_low().unwrap();

        rprintln!("Green On!");
        green_led.set_high().unwrap();
        timer.delay_ms(250_u32);
        green_led.set_low().unwrap();

        rprintln!("Blue On!");
        blue_led.set_high().unwrap();
        timer.delay_ms(250_u32);
        blue_led.set_low().unwrap();
    }

}