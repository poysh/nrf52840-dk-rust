#![no_main]
#![no_std]

use playground::{self as _}; // global logger + panicking-behavior + memory layout
use playground::{
    dk_button, 
    rgb_led,
    number_representation::{self, Unit},
}; 
use nb::block;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    prelude::*,
    Temp, Timer,
    gpio::{p0::Parts as P0Parts},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();

    // one for continuous counting
    let mut periodic_timer = Timer::periodic(board.TIMER0);
    let mut millis: u64 = 0;

    let pins = P0Parts::new(board.P0);

    let led_channel_red = pins.p0_13.degrade(); //onboard
    let led_channel_green = pins.p0_14.degrade(); //onboard
    let led_channel_blue = pins.p0_15.degrade(); //onboard

    let mut light = rgb_led::LEDColour::init(led_channel_red, led_channel_green, led_channel_blue);
    let mut temp_sensor = Temp::new(board.TEMP);
    let mut button_1 = dk_button::Button::new(pins.p0_11.degrade());

    let lower_limit: f32 = 22.0;
    let upper_limit: f32 = 25.0;

    // state of the button is read and updated continuoulsly
    // but temp value is only printed if tick number is divisible

    let mut current_unit = number_representation::Unit::Celsius;

    loop {
        // Start by setting/resetting the timer for next interval
        // Timer counts in microseconds/at 1MHz, we care about milliseconds.
        periodic_timer.start(1000u32);

        // Every 1000ms:
        // read temperature
        // light led in appropriate color
        // print the current temperature reading

        if (millis % 1000) == 0 {
            defmt::info!("Tick (milliseconds): {:u32}", millis as u32);

            let temperature: f32 = temp_sensor.measure().to_num();

            if temperature > lower_limit && temperature < upper_limit {
                light.green();
            } else if temperature <= lower_limit {
                light.blue();
            } else {
                light.red();
            }

            let converted_temp = current_unit.convert_temperature(&temperature);
            match current_unit {
                Unit::Fahrenheit => defmt::info!("{:f32} °F", converted_temp),
                Unit::Kelvin => defmt::info!("{:f32} K", converted_temp),
                Unit::Celsius => defmt::info!("{:f32} °C", converted_temp),
            };
        };

        // Every 5ms, check the current state of the button
        if (millis % 5) == 0 && button_1.check_rising_edge() {
            current_unit = match current_unit {
                Unit::Fahrenheit => Unit::Kelvin,
                Unit::Kelvin => Unit::Celsius,
                Unit::Celsius => Unit::Fahrenheit,
            };
        };

        // Now wait for the timer to complete
        block!(periodic_timer.wait()).unwrap();

        // Increment our millisecond count
        millis = millis.wrapping_add(1);
    }
}