#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use microbit::hal::prelude::*;
use microbit::hal::gpio::{p0::Parts as P0Parts, Level};
use microbit::hal::pac::Peripherals;
//use microbit::hal::pac::gpio;
//use microbit::hal::pac::gpio::Level;
use microbit::hal::pac::PWM0;
use microbit::hal::pwm::{self, Pwm, Channel, Prescaler};
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::time::Hertz;

#[entry]
fn main() -> ! {
    rtt_init_print!();

     /* 
    Will work out Pulse Width Modulation example at a later time.
    
    rprintln!("Program start!");

   
    let board = Peripherals::take().expect("Couldn't initialize board.");
    let gpio = P0Parts::new(board.P0);

    // Set up PWM for RGB
    let pwm_motor = pwm::Pwm::new(board.PWM0);
    let pwm_pin_red = gpio.p0_10.into_push_pull_output(Level::Low).degrade();
    let pwm_pin_green = gpio.p0_09.into_push_pull_output(Level::Low).degrade();
    let pwm_pin_blue = gpio.p0_07.into_push_pull_output(Level::Low).degrade();

    // Set up output Pins
    pwm_motor.set_output_pin(pwm::Channel::C0, pwm_pin_red);
    pwm_motor.set_output_pin(pwm::Channel::C1, pwm_pin_green);
    pwm_motor.set_output_pin(pwm::Channel::C2, pwm_pin_blue);

    // Define the duty cycles
    let duty_red: u16 = 5; 
    let duty_green: u16 = 7;
    let duty_blue: u16 = 3;

    pwm_motor.set_prescaler(pwm::Prescaler::Div32);

    pwm_motor.set_max_duty(10_u16);

    //Ensure program keeps running
    pwm_motor.set_duty_off(pwm::Channel::C0, duty_red);
    rprintln!("Red On!");
    pwm_motor.set_duty_off(pwm::Channel::C1, duty_green);
    rprintln!("Green On!");
    pwm_motor.set_duty_off(pwm::Channel::C2, duty_blue);
    rprintln!("Blue On!");
 
    pwm_motor.loop_inf();

    */

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