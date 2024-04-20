# SSD3306 I2C &emsp; 
[![crates.io](https://img.shields.io/crates/v/ssd1306-i2c)](https://crates.io/crates/ssd1306-i2c)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/marvinrobot42/ssd1306-i2c)
[![Documentation](https://docs.rs/ssd1306-i2c/badge.svg)](https://docs.rs/ssd1306-i2c)

## A Rust crate for SSD1306 LCD display over I<sup>2</sup>C 

<https://github.com/marvinrobot42/ssd1306-i2c.git>

## [Documentation](https://docs.rs/ssd1306-i2c/latest/ssd1306_i2c/)


[ssd1306]: https://www.digikey.com/htmldatasheets/production/2047793/0/0/1/ssd1306.html#pf13

This crate is a fork of James Waples's SH1106 crate modifed for SSD1306 use updated to use 
embedded-hal 1.0.

The SSD1306 LCD contains old fashion 8 bit parallel interface, a SPI and I2C interface.  This crate
only uses the I2C interface.  Also, note that this driver includes support for several display sizes 
but only 128 x 64 is avialble for the SSD1306 (as far as I know).


### Features

- updated to use embedded-hal version 1.0.x
- supports embedded-hal-bus 0.1.0 for I2C bus sharing
- uses embedded-graphics crate for graphics abstraction
- designed for embedded use (ESP32-C3, -C6 and -S3)
- supports both primary and secondary I2C SSD1306 addresses
- no_std embedded compatible

  

#### Notes

I wanted to add a LCD display to my home automation environmental sensor IoT creation but I could not
find a crate for the SSD1306 that depends on embedded-hal 1.0.  It needs to have several devices on one
I2C bus and since my sensors are embedded-hal 1.0 based the SSD1306 crate needs to be also.  So I did a
quick job of migrating James's SH1106 crate from embedded-hal 0.2.x to 1.0, and modified it for the SSD1306 LCD.  Why start with SH1106 instead of his SSD1306 create: because the SH1106 seemed more modern and does not depend on other embedded-hal 0.2 crates like display-interface-i2c (which is four years old now).  Note that this ssd1306-i2c crate does not support SH1106 display because the display initialization is not compatible.

My Sparkfun SSD1306 LCD (LCD-23453) only supports I2C so I could not work with or test SPI.

### Recent version history
  - 0.1.3  More documentation
  - 0.1.1  Some document typo fixes
  - 0.1.0  Initial release


## Usage
----

Add the dependencies to `Cargo.toml`.

~~~~toml
[dependencies]
ssd1306-i2c version = "0.1"
embedded-graphics = "0.8.1" 
esp-idf-hal = "0.43.0"  # an embedded-hal for your platform
~~~~
 


### Simple Example

A more complete example is in the repository examples path.  See James's SSH1106 crate repository
examples folder for more examples: https://github.com/jamwaffles/sh1106/tree/master/examples.

```rust

use ssd1306_i2c::{prelude::*, Builder}; 

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, ascii::FONT_6X13_BOLD, MonoTextStyleBuilder},
    image::{Image, ImageRawLE},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use log::info;

...


fn main() -> Result<()> {
  // Bind the log crate to the ESP Logging facilities
  esp_idf_svc::log::EspLogger::initialize_default();
  ...

  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;
  let sda = pins.gpio0;
  let scl = pins.gpio1;
  let i2c = peripherals.i2c0;
  let config = I2cConfig::new().baudrate(400.kHz().into());
  let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

  let mut display: GraphicsMode<_> = Builder::new()
    .with_size(DisplaySize::Display128x64NoOffset)
    .with_i2c_addr(0x3d)  //or 0x3c
    .with_rotation(DisplayRotation::Rotate0)
    .connect_i2c(i2c_dev)
    .into();

  info!("calling display.init()");
  FreeRtos::delay_ms(100);
  display.init().unwrap();

  display.flush().unwrap();
 
  display.clear();

  //********* display some text

  // creating MonoTextStyleBuilder
  let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();

  let text_style_bold = MonoTextStyleBuilder::new()
    .font(&FONT_6X13_BOLD)
    .text_color(BinaryColor::On)
    .build();

  info!("displaying Hello world! on LCD");
  Text::with_baseline("Hello Rust World!....", Point::zero(), text_style_bold, Baseline::Top)
    .draw(&mut display)
    .unwrap();
  info!("displaying Hello Rust! on LCD");
  Text::with_baseline("SSD1306-I2C", Point::new(0, 19), text_style, Baseline::Top)
    .draw(&mut display)
    .unwrap();

  display.flush().unwrap();


  loop {
    info!("looping with delay so ESP32 watchdog does not restart controller");

    FreeRtos::delay_ms(10000);
  }

}
    
```


### License
----

You are free to copy, modify, and distribute this application with attribution under the terms of either

 * Apache License, Version 2.0
   ([LICENSE-Apache-2.0](./LICENSE-Apache-2.0) or <https://opensource.org/licenses/Apache-2.0>)
 * MIT license
   ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

This project is not affiliated with nor endorsed in any way by Sparkfun.
