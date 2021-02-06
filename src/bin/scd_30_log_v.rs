#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use playground::scd30;
use playground::buzzer;
use nrf52840_hal::{self as hal, 
    gpio::{
        p0::{
            Parts as P0Parts
    }, 
    Level,
},
prelude::*, 
Timer,
twim::{self, Twim},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let pins = P0Parts::new(board.P0);
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);

    // Buzzer
    let buzzer_pin = pins.p0_29.degrade();
    let mut buzzer = buzzer::Buzzer::init(buzzer_pin);
    buzzer.noise(&mut timer);
    

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

    // set pressure to local value
    let pressure = 1015_u16;

    // start continuous measurement
    sensor.start_continuous_measurement(pressure).unwrap();

    loop {
        led_1.set_high().unwrap();
        timer.delay(250_000);

        if sensor.data_ready().unwrap() {
            led_1.set_low().unwrap();
            timer.delay(250_000);

            let result = sensor.read_measurement().unwrap();

            let co2 = result.co2;
            let temperature = result.temperature;
            let humidity = result.humidity;

            defmt::info!("
                CO2 {:f32} ppm
                Temperature {:f32} C
                Humidity {:f32} %
                ", co2, temperature, humidity
            );
        }
    }


    //playground::exit()
}
