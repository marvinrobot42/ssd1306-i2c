// ESP32-C6 based example

/**** Cargo.toml includes:

[dependencies]
anyhow = "1.0.75"
log = { version = "0.4", default-features = false }
esp-idf-svc = { version = "0.48", default-features = false }
esp-idf-hal = "0.43.0"
esp-idf-sys = { version = "0.34.0", features = ["binstart"] }
sths34pf80 = { version = "0.1"}

[build-dependencies]
embuild = "0.31.3"

****************/

use anyhow::Result;

use esp_idf_hal::{
    delay::{Ets, FreeRtos}, i2c::{I2cConfig, I2cDriver}, io::Error, prelude::*
};
use esp_idf_hal::{gpio::PinDriver, prelude::Peripherals};
use esp_idf_sys::{self as _};


use log::info;
use sths34pf80::{data::{AverageTrimT1Flag, AverageTrimTMOSFlag, Gain, InterruptPinConfig, Measurements}, Sths34pf80};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    log::debug!("a debug message");

    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;
    let sda = pins.gpio6; // esp32-c3  has pins.gpio0;
    let scl = pins.gpio7; // esp32-c3  haspins.gpio1;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

    // STHS34PF80 INT pin to connected to ESP32-C6 GIO pin 23 (just read, no IRQ handler configured)
    let mut led_pin23 = PinDriver::output(pins.gpio23).unwrap();

    let mut sths34pf80 = Sths34pf80::new(i2c_dev, Ets{});
    sths34pf80.initialize().unwrap();
   
    info!("     calling get_presence_threshold()");
    let presence_threshold = sths34pf80.get_presence_threshold().unwrap();
    info!(" presence threshold is {}", presence_threshold);
 

    sths34pf80.set_tmotion_threshold_new(500).unwrap();  // default is 200
    let tmotion_threshold = sths34pf80.get_tmotion_threshold().unwrap();
    info!(" tmotion threshold is {}",tmotion_threshold);

    /**** all below is tested ***************************
    sths34pf80.set_tambient_shock_threshold_new(253).unwrap();
    let tamb_shock_threshold = sths34pf80.get_tambient_shock_threshold().unwrap();
    info!(" tambient shock threshold is {}", tamb_shock_threshold);

   
    sths34pf80.set_presence_threshold_new(1200).unwrap();
    presence_threshold = sths34pf80.get_presence_threshold().unwrap();
    info!(" after set_presence_threshold(1200) presence threahold is {}", presence_threshold);

    sths34pf80.set_gain_mode(Gain::GainDefault);
    let gain_mode: Gain = sths34pf80.get_gain_mode().unwrap();
    info!(" gain mode is {:#?}", gain_mode);

    sths34pf80.set_avg_tambient_num(AverageTrimT1Flag::AVG_T8).unwrap();
    let avg_tambient_num: AverageTrimT1Flag = sths34pf80.get_avg_tambient_num().unwrap();
    info!(" avg_tambient_num is {:#?}", avg_tambient_num);

    let tmos_sens = sths34pf80.get_tmos_sensitivty().unwrap();
    info!("get_tmos_sensivity is : {}", tmos_sens);

    sths34pf80.set_tmos_sensitivity(0.0).unwrap();  // 0 -to- +4080
    
    ********/

    // sths34pf80.set_avg_tmos_num(AverageTrimTMOSFlag::AVG_TMOS32).unwrap();

    let int_pin_config = InterruptPinConfig::builder()
        .active_high()
        .open_drain()
        //.on_new_any()
        .on_new_presence()
        .latched_pin()  // testing: the method ien_int_or will clear this bit
        .ien_int_or()
        .build();  // build method just returns this struc's .0 (u8) property
        
    info!("setting to int_pin_config value {:#04x}", int_pin_config);
    if let Ok(new_int_config) = sths34pf80.set_config_interrupt_pin(int_pin_config) {
        if (new_int_config.get_u8_value() == int_pin_config) {
            info!("interrupt pin config changed as expected to {:#04x}", new_int_config.0);
        } else {
            log::error!("interrupt pin config change problem, expected {:#04x}, got {:#04x}", 
            int_pin_config, new_int_config.get_u8_value());
        }
    }
    

    loop {
        led_pin23.set_high().unwrap();
        log::debug!("\n\nLED is high");  // flash LED before reading measurements
        FreeRtos::delay_ms(500);


        // sth34pf80
        if let Ok(measurements) = sths34pf80.get_measurements_timeout(10) {
            match measurements.presence_value {
                Some(presence_value) => info!("presence detected, value is {}", presence_value),
                _ => (),
            }
            match measurements.motion_value {
                Some(motion_value) => info!("motion detected, value is {}", motion_value),
                _ => (),
            }
            match measurements.ambient_shock_value {
                Some(abmient_shock_value) => info!("ambient shock temp detected, value is {}", abmient_shock_value),
                _ => (),
            }
        } else {
            info!("timeout with get_measurements_timeout call");
        }

        /************* or read individually
        if let Ok(data_ready) = sths34pf80.get_data_ready() {
            if (data_ready) {
                if let Ok(func_status) = sths34pf80.get_func_status() {
                    info!("sth34pf80 data ready, func status is {:#?}", func_status);
                    if (func_status.presence_flag()) {
                        let presence_val = sths34pf80.get_presence().unwrap();
                        info!(" presence detected! presence value is {} /cm", presence_val);
                    }
                    if (func_status.mot_flag()) {
                        let tmotion_val = sths34pf80.get_tmotion().unwrap();
                        info!(" motion detected!  tmotion value is {} C ", tmotion_val);
                    }
                    if (func_status.tamb_shock_flag()) {
                        let tambient_shock_val = sths34pf80.get_shock_temperature().unwrap();
                        info!("  ambient shock detected! tambient shock temperature is {} C", tambient_shock_val);
                    }
                }
                // info!("get_tobject_raw_in_c is {}", sths34pf80.get_tobject_raw_in_c().unwrap());
                
            } else {
                info!("sth34pf80 data not ready");
            }
        }

        ******/

        led_pin23.set_low().unwrap();
        log::debug!("LED is low\n\n");
        FreeRtos::delay_ms(5000);
    }
}

