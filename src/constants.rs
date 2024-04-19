// STHS34PF80 registers
pub const STHS34PF80_LPF1: u8 = 0x0c;
pub const STHS34PF80_LPF2: u8 = 0x0d;
pub const STHS34PF80_WHO_AM_I: u8 = 0x0f;
pub const STHS34PF80_AVG_TRIM: u8 = 0x10;
pub const STHS34PF80_CTRL0: u8 = 0x17;
pub const STHS34PF80_SENS_DATA: u8 = 0x1d;
pub const STHS34PF80_CTRL1: u8 = 0x20;
pub const STHS34PF80_CTRL2: u8 = 0x21;
pub const STHS34PF80_CTRL3: u8 = 0x22;
pub const STHS34PF80_STATUS: u8 = 0x23;
pub const STHS34PF80_FUNC_STATUS: u8 = 0x25;
pub const STHS34PF80_TOBJECT_L: u8 = 0x26;
pub const STHS34PF80_TOBJECT_H: u8 = 0x27;
pub const STHS34PF80_TAMBIENT_L: u8 = 0x28;
pub const STHS34PF80_TAMBIENT_H: u8 = 0x29;
pub const STHS34PF80_TOBJ_COMP_L: u8 = 0x38;
pub const STHS34PF80_TOBJ_COMP_H: u8 = 0x39;
pub const STHS34PF80_TPRESENCE_L: u8 = 0x3a;
pub const STHS34PF80_TPRESENCE_H: u8 = 0x3b;
pub const STHS34PF80_TMOTION_L: u8 = 0x3c;
pub const STHS34PF80_TMOTION_H: u8 = 0x3d;
pub const STHS34PF80_TAMB_SHOCK_L: u8 = 0x3e;
pub const STHS34PF80_TAMB_SHOCK_H: u8 = 0x3f;

// embedded function registers
pub const STHS34PF80_RESET_ALGO: u8 = 0x2a;
pub const STHS34PF80_PRESENCE_THS: u8 = 0x20;  // 2 bytes, includes 0x21
pub const STHS34PF80_MOTION_THS: u8 = 0x22; // 2 bytes
pub const STHS34PF80_TAMB_SHOCK_THS: u8 = 0x24; // 2 bytes
pub const STHS34PF80_PAGE_RW: u8 = 0x11;
pub const STHS34PF80_FUNC_CFG_ADDR: u8 = 0x08;
pub const STHS34PF80_FUNC_CFG_DATA: u8 = 0x09;
pub const STHS34PF80_HYST_MOTION: u8 = 0x26;
pub const STHS34PF80_HYST_PRESENCE: u8 = 0x27;
pub const STHS34PF80_HYST_TAMB_SHOCK: u8 = 0x29;


// constant values

pub const STHS34PF80_PART_ID: u8 = 0xd3;

#[repr(u8)]
/// STHS34PF80 I2C device address
/// do not float the ADDR pin as its value would be undefined.  Check your STHS34PF80 board specs.
#[derive(Debug, Clone, Copy)]
pub enum DeviceAddress {
    /// it only has one I2C address but let's call it primary in case future secondary is added
    Primary = 0x5a,  
}

impl From<DeviceAddress> for u8 {
    fn from(value: DeviceAddress) -> Self {
        match value {
            DeviceAddress::Primary => 0x5a,
        }
    }
}

impl Default for DeviceAddress {
    fn default() -> Self {
        Self::Primary
    }
}
