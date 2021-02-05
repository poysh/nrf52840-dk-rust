#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use playground::scd30;
use nrf52840_hal::{self as hal, 
    gpio::{
        p0::{
            Parts as P0Parts
    }, 
    Level,
},
prelude::*,
Temp, 
Timer,
twim::{self, Twim},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let pins = P0Parts::new(board.P0);
    let mut led_1 = pins.p0_03.into_push_pull_output(Level::Low);

    // instanciate I2C
    let scl = pins.p0_30.degrade().into_floating_input();
    let sda = pins.p0_31.degrade().into_floating_input();

    let pins = twim::Pins { scl, sda };
    let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);
    let mut sensor = scd30::SDC30::init(i2c);
    
    let firmware_version = sensor.get_firmware_version().unwrap();
    defmt::info!(
        "Firmware Version: {:u8}.{:u8}",
        firmware_version[0],
        firmware_version[1]
    );

    loop {
        timer.delay(250_000);
        led_1.set_high().unwrap();
        timer.delay(250_000);
        led_1.set_low().unwrap();
    }


    // playground::exit()
}
