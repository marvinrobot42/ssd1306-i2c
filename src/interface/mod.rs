
//! ssd1306 Communication Interface (I2C only)
pub mod i2c;

use crate::Error;

/// A method of communicating with sh1106
pub trait DisplayInterface {
    /// Interface error type
    type Error;

    /// Initialize device.
    fn init(&mut self) -> Result<(), Self::Error>;
    /// Send a batch of up to 8 commands to display.
    fn send_commands(&mut self, cmd: &[u8]) -> Result<(), Self::Error>;
    /// Send data to display.
    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

pub use self::i2c::I2cInterface;
