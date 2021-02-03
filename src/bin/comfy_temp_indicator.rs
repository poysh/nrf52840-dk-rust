#![no_main]
#![no_std]

use playground as _; // global logger + panicking-behavior + memory layout
use nrf52840_hal::{
    self as hal,
    prelude::*,
    Timer,
    Temp, 
    gpio::{p0::Parts as P0Parts,         
        Level, 
        Output,
        Input,
        PullUp, 
        PushPull,
        Pin,
}
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
    let mut timer = Timer::new(board.TIMER0);

    let pins = P0Parts::new(board.P0);

    let led_channel_red = pins.p0_13.degrade(); //onboard
    let led_channel_green = pins.p0_14.degrade(); //onboard
    let led_channel_blue = pins.p0_15.degrade(); //onboard

    let mut light = LEDColour::init(led_channel_red, led_channel_green, led_channel_blue);
    let mut temp_sensor = Temp::new(board.TEMP);
    let button_1 = Button::new(pins.p0_11.degrade());

    let lower_limit: f32 = 22.0;
    let upper_limit: f32 = 25.0;

    loop {
        if button_1.is_pressed() {
            let temperature: f32 = temp_sensor.measure().to_num();
            if temperature > lower_limit && temperature < upper_limit {
                light.green();
                defmt::info!("comfy! {:?}", temperature);
            } else if temperature <= lower_limit {
                light.blue();
                defmt::info!("cold! {:?}", temperature);
            } else {
                light.red();
                defmt::info!("hot! {:?}", temperature);
            }
            timer.delay_ms(1000u32);
        }
        
    }

    // playground::exit()
}
