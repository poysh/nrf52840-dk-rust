#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal::{self as hal, gpio::{p0::Parts as P0Parts, Level,}, Timer,};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let pins = P0Parts::new(board.P0);

    let mut led_red = pins.p0_03.into_push_pull_output(Level::Low);
    let mut led_green = pins.p0_04.into_push_pull_output(Level::Low);
    let mut led_blue = pins.p0_28.into_push_pull_output(Level::Low);

    timer.delay_ms(1000u32);

    loop {
        led_red.set_high().unwrap();
        timer.delay_ms(1000u32);
        led_red.set_low().unwrap();
        timer.delay_ms(1000u32);
        led_green.set_high().unwrap();
        timer.delay_ms(1000u32);
        led_green.set_low().unwrap();
        timer.delay_ms(1000u32);
        led_blue.set_high().unwrap();
        timer.delay_ms(1000u32);
        led_blue.set_low().unwrap();
        timer.delay_ms(1000u32);
        break;
    }

    playground::exit()
}
