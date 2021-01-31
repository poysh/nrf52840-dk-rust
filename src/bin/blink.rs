#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal as hal;
use hal::{gpio::Level, pac, prelude::{InputPin, OutputPin}};
use hal::temp::Temp;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("initializing");

    // setup peripherals
    let p = hal::pac::Peripherals::take().unwrap();

    // setup GPIO P0 side
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    // setup all four buttons
    let button1 = port0.p0_11.into_pullup_input();
    let button2 = port0.p0_12.into_pullup_input();
    let button3 = port0.p0_24.into_pullup_input();
    let button4 = port0.p0_25.into_pullup_input();

    // setup all 4 on board leds on GPIO P0
    let mut led1 = port0.p0_13.into_push_pull_output(Level::Low);
    let mut led2 = port0.p0_14.into_push_pull_output(Level::Low);
    let mut led3 = port0.p0_15.into_push_pull_output(Level::Low);
    let mut led4 = port0.p0_16.into_push_pull_output(Level::Low);

    // Access to the temp sensor
    let mut temp_sensor = Temp::new(p.TEMP);

    // safe temperature and convert it to a i32
    let die_temp_c = temp_sensor.measure().to_num();
    defmt::info!("die tempterature is : {:i32}", die_temp_c);


    loop {
        match button1.is_high().unwrap() {
            true => led1.set_high().unwrap(),
            false => led1.set_low().unwrap(),
        };

        match button2.is_high().unwrap() {
            true => led2.set_high().unwrap(),
            false => led2.set_low().unwrap(),
        };

        match button3.is_high().unwrap() {
            true => led3.set_high().unwrap(),
            false => led3.set_low().unwrap(),
        };

        match button4.is_high().unwrap() {
            true => led4.set_high().unwrap(),
            false => led4.set_low().unwrap(),
        };
    }

    //playground::exit()
}