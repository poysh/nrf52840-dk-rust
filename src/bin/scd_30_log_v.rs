#![no_main]
#![no_std]

use playground::{self as _, alert}; // global logger + panicking-behavior + memory layout
use playground::{scd30, buzzer, rgb_led};
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

    let channel_red = pins.p0_03.degrade();
    let channel_green = pins.p0_04.degrade();
    let channel_blue = pins.p0_28.degrade();

    let mut rgb_led = rgb_led::LEDColour::init(channel_red, channel_green, channel_blue);

    // Buzzer
    let buzzer_pin = pins.p0_29.degrade();
    let mut buzzer = buzzer::Buzzer::init(buzzer_pin);
    buzzer.noise(&mut timer);
    

    // instanciate I2C
    let scl = pins.p0_30.degrade().into_floating_input();
    let sda = pins.p0_31.degrade().into_floating_input();

    let pins = twim::Pins { scl, sda };
    let i2c = Twim::new(board.TWIM0, pins, twim::Frequency::K100);

    // set pressure to local value
    let pressure = 1015_u16;

    // Create a sensor I2C instance
    let mut sensor = scd30::SDC30::init(i2c);
    
    let firmware_version = sensor.get_firmware_version()
    .unwrap_or_else(|error| {
        rgb_led.error_blink_red(&mut timer);
        panic!("Error getting firmware version: {:?}", error)
    });

    defmt::info!(
        "Firmware Version: {:u8}.{:u8}",
        firmware_version[0],
        firmware_version[1]
    );


    // start continuous measurement
    sensor.start_continuous_measurement(pressure).unwrap();
    let mut send_alert = false;

    loop {
        led_1.set_high().unwrap();
        timer.delay_ms(2000_u32);

        if sensor.data_ready().unwrap() {
            led_1.set_low().unwrap();
            timer.delay_ms(2000_u32);

            let result = sensor.read_measurement().unwrap();

            let co2 = result.co2;
            let temperature = result.temperature;
            let humidity = result.humidity;

            alert::check_level(&co2, &mut buzzer, &mut rgb_led, &mut timer, &mut send_alert);

            defmt::info!("
                CO2: {:f32} ppm
                Temperature: {:f32} C
                Humidity: {:f32} %
                ", co2, temperature, humidity
            );
        }
    }


    //playground::exit()
}
