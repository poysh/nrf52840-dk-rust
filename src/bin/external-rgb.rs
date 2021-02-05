#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal::{
    prelude::*,
    gpio::{
        Level, 
        Output, 
        PushPull,
        Pin,
    }, 
    Timer,
};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

struct LEDColour {
    r: Pin<Output<PushPull>>,
    g: Pin<Output<PushPull>>,
    b: Pin<Output<PushPull>>,
}

impl LEDColour {

    pub fn init<Mode>(led_red: Pin<Mode>, led_green: Pin<Mode>, led_blue: Pin<Mode>) -> LEDColour {

        LEDColour {
            r: led_red.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
        }
    }

    fn red(&mut self) {
        self.r.set_low().unwrap();
        self.g.set_high().unwrap();
        self.b.set_high().unwrap();
    }

    fn green(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_low().unwrap();
        self.b.set_high().unwrap();
    }

    fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.g.set_high().unwrap();
        self.b.set_low().unwrap();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = nrf52840_hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let pins = nrf52840_hal::gpio::p0::Parts::new(board.P0);

    let led_channel_red = pins.p0_03.degrade(); //onboard
    let led_channel_green = pins.p0_04.degrade(); //onboard
    let led_channel_blue = pins.p0_28.degrade(); //onboard

    let mut light = LEDColour::init(led_channel_red, led_channel_green, led_channel_blue);

    timer.delay_ms(1000u32);

    loop {
        light.red();
        timer.delay_ms(1000u32);
        light.blue();
        timer.delay_ms(1000u32);
        light.green();
        timer.delay_ms(1000_u32);
    }

    // playground::exit()
}
