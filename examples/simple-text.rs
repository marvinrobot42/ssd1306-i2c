// ESP32-C6 style ssd1306-i2c example to display text lines
/****  Cargo.toml  example

authors = ["your email address"]
edition = "2021"
resolver = "2"
rust-version = "1.71"

[profile.release]
opt-level = "s"

[profile.dev]
debug = true    # Symbols are nice and they don't increase the size on Flash
opt-level = "z"

[features]
default = ["std", "embassy", "esp-idf-svc/native"]

pio = ["esp-idf-svc/pio"]
std = ["alloc", "esp-idf-svc/binstart", "esp-idf-svc/std"]
alloc = ["esp-idf-svc/alloc"]
nightly = ["esp-idf-svc/nightly"]
experimental = ["esp-idf-svc/experimental"]
embassy = ["esp-idf-svc/embassy-sync", "esp-idf-svc/critical-section", "esp-idf-svc/embassy-time-driver"]

[dependencies]
anyhow = "1.0.75"
log = { version = "0.4", default-features = false }
esp-idf-svc = { version = "0.48", default-features = false }
esp-idf-sys = { version = "0.34.0", features = ["binstart"] }
esp-idf-hal = "0.43.0"

embedded-graphics = "0.8.1"  # latest is 0.8.1
ssd1306-i2c = { version = "0.1"}

[build-dependencies]
embuild = "0.31.3"
 
 
 */


use anyhow::Result;

use esp_idf_hal::{
    delay::{Ets, FreeRtos}, i2c::{I2cConfig, I2cDriver}, io::Error, prelude::*
};
use esp_idf_hal::prelude::Peripherals;

use esp_idf_sys::{self as _};
use log::info;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, ascii::FONT_6X13_BOLD, MonoTextStyleBuilder},
    image::{Image, ImageRawLE},  // not needed for just text
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use ssd1306_i2c::{prelude::*, Builder};  // was use sh1106:: ...


fn main() -> Result<()> {

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    FreeRtos::delay_ms(1000);
    log::info!("Hello, SSD1306-I2C world!");

    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;
    let sda = pins.gpio6; // esp32-c3  has pins.gpio0;
    let scl = pins.gpio7; // esp32-c3  haspins.gpio1;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());  // works ok at 400 kHz with short bus length
    let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

    // create and ssd1306-i2c instance using builder
    let mut display: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x64NoOffset)
        .with_i2c_addr(0x3d)  // your LCD may used 0x3c the primary address
        .with_rotation(DisplayRotation::Rotate0)
        .connect_i2c(i2c_dev)
        .into();

    info!("calling display.init()");
    FreeRtos::delay_ms(100);
    display.init().unwrap();

    FreeRtos::delay_ms(100);

    info!("calling display.flush()");
    display.flush().unwrap();

    //*********  some text
    info!("creating MonoTextStyleBuilder ");
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let text_style_bold = MonoTextStyleBuilder::new()
        .font(&FONT_6X13_BOLD)
        .text_color(BinaryColor::On)
        .build();

    info!("displaying Hello world! on LCD");
    Text::with_baseline("Hello world!.........", Point::zero(), text_style_bold, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    info!("displaying Hello Rust! on LCD");
    Text::with_baseline("Hello Rust!", Point::new(0, 19), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("SH1106 crate modified", Point::new(0, 31), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    Text::with_baseline("for SSD1306 use", Point::new(0, 43), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    Text::with_baseline("embedded-hal 1.0 !", Point::new(0, 56), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    //******************************/
    loop {
        info!("looping.........");
        FreeRtos::delay_ms(5000);  // allows watchdog to reset
    }

    //Ok(())
}