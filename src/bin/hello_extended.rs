#![no_main]
#![no_std]

use BinaryColor::On;
use playground as _; // global logger + panicking-behavior + memory layout
// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, p1::Parts as P1Parts, Level},
    prelude::*,
    spim::{self, Spim},
    Timer,
};
use epd_waveshare::{
    epd2in9bc::*,
    graphics::Display,
    prelude::*,
};

use embedded_graphics::{geometry::Point, pixelcolor::BinaryColor, prelude::*, primitives::{Circle, Line, Triangle}, style::PrimitiveStyle};

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    // let mut timer = Timer::new(board.TIMER0);

    let pins_1 = P1Parts::new(board.P1);

    let din = pins_1.p1_01.into_push_pull_output(Level::Low).degrade();
    let clk = pins_1.p1_02.into_push_pull_output(Level::Low).degrade();
    let cs = pins_1.p1_03.into_push_pull_output(Level::Low);
    let dc = pins_1.p1_04.into_push_pull_output(Level::Low);
    let rst = pins_1.p1_05.into_push_pull_output(Level::Low);
    let busy = pins_1.p1_06.into_floating_input();

    let spi_pins = spim::Pins {
        sck: clk,
        miso: None,
        mosi: Some(din),
    };

    let mut spi = Spim::new(board.SPIM3, spi_pins, spim::Frequency::K500, spim::MODE_0, 0);

    let mut delay  = Timer::new(board.TIMER1);
    let mut epd2in9 = EPD2in9bc::new(&mut spi, cs, busy, dc, rst, &mut delay).unwrap();

    let mut display = Display2in9bc::default();

    let _ = Line::new(Point::new(0, 120), Point::new(0, 200))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display);

    let c1 = Circle::new(Point::new(30, 30), 30)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display);

    let c2 = Circle::new(Point::new(40, 50), 30)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display);

    let t1 = Triangle::new(Point::new(38, 89), Point::new(25, 120), Point::new(100, 100))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display);

    epd2in9.update_frame(&mut spi, &display.buffer()).unwrap();
    epd2in9.display_frame(&mut spi)
        .expect("display frame new graphics");

    playground::exit()
}