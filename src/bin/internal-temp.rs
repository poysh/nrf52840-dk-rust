#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal::{self as hal, Temp, Timer,};
use embedded_hal::{blocking::delay::DelayMs};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut temp_sensor = Temp::new(board.TEMP);

    loop { 
        let temperature: f32 = temp_sensor.measure().to_num();
        defmt::info!("Temperature: {:?}C", temperature);
        timer.delay_ms(60000u32);
    }

    //playground::exit()
}
