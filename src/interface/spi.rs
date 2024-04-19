//! sh1106 SPI interface
//! 
/// SPI has not been migrated to embedded-hal 1.0,  do not attempt to include as is
/// it is left here as a ToDo.  I have no way to test SPI with SSD1306 display presently. 

use hal::{self, digital::v2::OutputPin};

use super::DisplayInterface;
use crate::Error;

/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC, CS> {
    spi: SPI,
    dc: DC,
    cs: CS,
}

impl<SPI, DC, CS, CommE, PinE> SpiInterface<SPI, DC, CS>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
    CS: OutputPin<Error = PinE>,
{
    /// Create new SPI interface for communciation with sh1106
    pub fn new(spi: SPI, dc: DC, cs: CS) -> Self {
        Self { spi, dc, cs }
    }
}

impl<SPI, DC, CS, CommE, PinE> DisplayInterface for SpiInterface<SPI, DC, CS>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin<Error = PinE>,
    CS: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.cs.set_high().map_err(Error::Pin)
    }

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        self.dc.set_low().map_err(Error::Pin)?;

        self.spi.write(&cmds).map_err(Error::Comm)?;

        self.dc.set_high().map_err(Error::Pin)?;
        self.cs.set_high().map_err(Error::Pin)
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;

        // 1 = data, 0 = command
        self.dc.set_high().map_err(Error::Pin)?;

        self.spi.write(&buf).map_err(Error::Comm)?;

        self.cs.set_high().map_err(Error::Pin)
    }
}
