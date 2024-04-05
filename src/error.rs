use core::fmt::Formatter;

use embedded_hal::i2c::{I2c, SevenBitAddress};

/// All possible errors 
/// Display not implemented for no_std support
pub enum Sths34pf80Error<I2C>
where
    I2C: I2c<SevenBitAddress>
{
    /// Error during I2C write operation.
    WriteError(I2C::Error),
    /// Error during I2C WriteRead operation.
    WriteReadError(I2C::Error),
    /// Got an unexpected Part Id during sensor initalization.
    UnexpectedChipId(u8),
    /// unexpected Operation Mode
    OpModeNotCorrect(u8),
    /// sths34pf80 device not connected to I2C bus
    NotConnected,
    /// Odr value out of range
    OdrNewTooBig(u8),
    /// threshold setting is too big
    ThresholdTooBig(u16),
    OutOfRange(),
    MeasurementTimeout(),
}

impl<I2C> core::fmt::Debug for Sths34pf80Error<I2C>
where
    I2C: I2c<SevenBitAddress>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Sths34pf80Error::WriteReadError(e) => f.debug_tuple("WriteReadError").field(e).finish(),
            Sths34pf80Error::WriteError(e) => f.debug_tuple("WriteError").field(e).finish(),
            Sths34pf80Error::UnexpectedChipId(chip_id) => f
                .debug_tuple("Expected part id 0xd3, got : ") // ToDo:  fix this one
                .field(chip_id)
                .finish(),
            Sths34pf80Error::OpModeNotCorrect(expected) => f
                .debug_tuple("Incorrect STHS34PF80 operation, got :")
                .field(expected)
                .finish(),
            Sths34pf80Error::NotConnected => f
                .debug_tuple("STH34PF80 is not connected to microcontroller")
                .finish(),
            Sths34pf80Error::OdrNewTooBig(value) => f
                .debug_tuple("ODR new value is too big for current AVG Trim TMOS, max = ")
                .field(value)
                .finish(),
            Sths34pf80Error::ThresholdTooBig(value) => f
                .debug_tuple("Threshold set value too big, must to less than 0x8000")
                .field(value)
                .finish(),
            Sths34pf80Error::OutOfRange() => f
                .debug_tuple("Set value out of range, check STHS34PF80 datasheet")
                .finish(),
            Sths34pf80Error::MeasurementTimeout() => f
                .debug_tuple("timeout waiting for new measurement data ready")
                .finish(),
        }
    }
}
