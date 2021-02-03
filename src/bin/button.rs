#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal::{prelude::*, self as hal, {gpio::{p0::Parts as P0Parts, Pin, Level, Input, Output, PullUp}}};
use embedded_hal::digital::v2::OutputPin;

pub struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }

    fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();

    let pins = P0Parts::new(board.P0);

    let button_1 = Button::new(pins.p0_11.degrade());
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::High);

    loop {
        match button_1.is_pressed() {
            true => led_1.set_low().unwrap(),
            false => led_1.set_high().unwrap(),
        }
    }

    //playground::exit()
}
