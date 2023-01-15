use rppal::i2c::I2c;
use sh1106::{prelude::*, Builder};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

fn main() -> () {
    //
    let i2c;
    match I2c::new() {
        Ok(r) => {
            i2c = r;
            eprintln!("Successfully created i2c interface");
        }
        Err(e) => {
            panic!("Failed to create i2c interface: {:?}", e);
        }
    }

    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    display.init().unwrap();
    display.flush().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello, Rust", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline(
        "embedded world!",
        Point::new(0, 8),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    display.flush().unwrap();
}
