//! ssd1306 I2C OLED display driver
//!

#![no_std]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

use embedded_hal::i2c::{Error as I2cError, ErrorKind as I2cErrorKind};

/// Errors in this crate
//#[derive(Debug)]
// pub enum Error<CommE, PinE> {
//     /// Communication error
//     Comm(CommE),
//     /// Pin setting error
//     Pin(PinE),
// }

#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// some other error
    OtherError,

    /// An error in the  underlying I²C system
    I2c(I2cErrorKind),
}

impl<E> From<E> for Error
where
    E: I2cError,
{
    fn from(error: E) -> Self {
        Self::I2c(error.kind())
    }
}

extern crate embedded_hal as hal;

pub mod builder;
mod command;
pub mod displayrotation;
pub mod displaysize;
pub mod interface;
pub mod mode;
pub mod prelude;
pub mod properties;
#[doc(hidden)]
//pub mod test_helpers;

mod brightness;

pub use crate::builder::Builder;
