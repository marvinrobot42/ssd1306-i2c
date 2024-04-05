# STHS34PF80 &emsp; [![crates.io](https://img.shields.io/crates/v/sths34pf80)](https://crates.io/crates/sths34pf80)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/marvinrobot42/sths34pf80)
[![Documentation](https://docs.rs/sths34pf80/badge.svg)](https://docs.rs/sths34pf80)

## A Rust crate for ST Microelectronics STHS34PF80 presence and motion detection sensor 

<https://github.com/marvinrobot42/sths34pf80.git>

[STHS34PF80]: https://www.st.com/en/mems-and-sensors/sths34pf80.html

The STHS34PF80 sensor detects infrared presence, meaning a warm "body" stationary object.  It also does ambient 
shock temperature detection (excessive heat change) and motion detection.

Many configurable settings for the detection thresholds, data output rates, interrupt pin, low pass filters,
sensitivity. 

### Features

- updated to use embedded-hal version 1.0.x
- designed for embedded use (ESP32-C3, -C6 and -S3 and STM32F3DISCOVERY)
- configurable interrupt pin
- data ready status based presence, motion and ambient shock measurement reads 
- low pass filter configuration in needed
- hysteresis getters and setters
- sensivity getters and setters
- gain getters and setters
- data output rate, average trim getters and setters
- an easy to use Measurements struct
- an easy to use initialize function
- easy to use get_measurement_blocking and get_measurement_timeout (with timeout parameter)
- no_std embedded compatible

  

## Notes

This driver is loosly based on STMicroelectronics STHS34PF80 driver.  A Sparkfun STHS34PF80 
sensor board was used for this driver devlopment.

### Recent version history
0.1.0  Initial release.


## Usage
----

Add the dependency to `Cargo.toml`.

~~~~toml
[dependencies.sths34pf80]
version = "0.1"
~~~~

Create a hardward specific I²C driver interface and delay function
Create an Sths34pf80 struct from the I²C interface and a delay function.
Configure interrupt pin properties if required.  
Initialize Sths34pf80  (initialize() fn sets "standard" trims and ODR paramaters).
Set presence, motion or ambient shock threshold only if required for more/less detection range
Read the STHS34PF80 status and check if new data is ready, then get_func_status and match
the returns FUNC_STATUS enum to see which of the three values changed, then get those values.  
 


## Simple Example

A more complete example is in the repository examples path
~~~~rust


use sths34pf80::{Sths34pf80};

...


fn main() -> Result<()> {

  ...

  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;
  let sda = pins.gpio0;
  let scl = pins.gpio1;
  let i2c = peripherals.i2c0;
  let config = I2cConfig::new().baudrate(100.kHz().into());
  let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

  let mut sths34pf80 = Sths34pf80::new(i2c_dev, Ets{});  // Ets is ESP32 IDF delay function

  sths34pf80.initialize().unwrap();  



  loop {
    if let Ok(status) = ens160.get_status() {
      if status.new_data_ready() {  // read all measurements
        let measuremnts: Measurements = ens160.get_measurements_blocking().unwrap();
        info!("measurements are : {:#?}\n\n", measuremnts);
      }    
      else {
        info!("no new data ready");
      }  
    }

    FreeRtos::delay_ms(10000);
  }

}
    
~~~~


License
----

You are free to copy, modify, and distribute this application with attribution under the terms of either

 * Apache License, Version 2.0
   ([LICENSE-Apache-2.0](./LICENSE-Apache-2.0) or <https://opensource.org/licenses/Apache-2.0>)
 * MIT license
   ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

This project is not affiliated with nor endorsed in any way by STMicroelectronics or Sparkfun.
