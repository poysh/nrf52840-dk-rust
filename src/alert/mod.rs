use nrf52840_hal::{pac::TIMER0, timer::OneShot,Timer};

use crate::{buzzer::Buzzer, rgb_led::LEDColour};

pub const WARN_LIMIT: f32 = 1000.0;
pub const UPPER_LIMIT: f32 = 1400.0;

pub fn check_level(
    co2: &f32,
    buzzer: &mut Buzzer,
    led: &mut LEDColour,
    mut timer: &mut Timer<TIMER0, OneShot>,
    alert: &mut bool,
) {
    if *co2 < WARN_LIMIT {
        led.green();
    } else if *co2 > WARN_LIMIT && *co2 < UPPER_LIMIT {
        led.yellow();
    } else {
        led.red();
        if *alert == false {
            buzzer.noise(&mut timer);
            *alert = true;
        }

    }
}