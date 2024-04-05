// no_std support 
#[allow(unused_imports)]
#[warn(dead_code)]
// use libm::{exp, round, trunc};
use log::debug;

#[allow(unused_imports)] // for no_std use
//use num_traits::float::FloatCore;

//use crate::error::Ens160Error;
use bitfield::bitfield;


/// A measurement result from the sensor.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Measurements {
    /// presence value if ready
    pub presence_value: Option<i16>,
    /// motion value if ready
    pub motion_value: Option<i16>,
    /// ambient shock temperature value if ready
    pub ambient_shock_value: Option<i16>,
}

// required by bitfield below
/// Average trim T1 values
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AverageTrimT1Flag {
    AVG_T8 = 0x00,
    AVG_T4 = 0x01,
    AVG_T2 = 0x02,
    AVG_T1 = 0x03,
}

// required by bitfield
impl From<u8> for AverageTrimT1Flag {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::AVG_T8,
            0x01 => Self::AVG_T4,
            0x02 => Self::AVG_T2,
            0x03 => Self::AVG_T1,
            _ => unreachable!(), 
        }
    }
}

// required by bitfield below
/// Average Trim TMOS values
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AverageTrimTMOSFlag {
    AVG_TMOS2 = 0x00,
    AVG_TMOS8 = 0x01,
    AVG_TMOS32 = 0x02,
    AVG_TMOS128 = 0x03,  // default
    AVG_TMOS256 = 0x04,
    AVG_TMOS512 = 0x05,
    AVG_TMOS1024 = 0x06,
    AVG_TMOS2048 = 0x07,
}

// required by bitfield
impl From<u8> for AverageTrimTMOSFlag {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::AVG_TMOS2,
            0x01 => Self::AVG_TMOS8,
            0x02 => Self::AVG_TMOS32,
            0x03 => Self::AVG_TMOS128,
            0x04 => Self::AVG_TMOS256,
            0x05 => Self::AVG_TMOS512,
            0x06 => Self::AVG_TMOS1024,
            0x07 => Self::AVG_TMOS2048,
            _ => unreachable!(), 
        }
    }
}

bitfield! {
    /// STHS34PF80 AVG_TRIM bits
    pub struct Avg_trim(u8);
    impl Debug;

    pub into AverageTrimT1Flag, average_trim_t1_flags, set_trim_t1: 5,4; // 2 bits
    
    pub into AverageTrimTMOSFlag, average_trim_tmos_flag, set_trim_tmos: 2,0;  // 3 bits
    // bits 7, 6 and 3 not used

}

// required by bitfield below
/// Output Data Rate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
pub enum Odr {
    ODR_POWERDOWN = 0x00,
    ODR_HZ025 = 0x01,
    ODR_HZ050 = 0x02,
    ODR_HZ1 = 0x03,
    ODR_HZ2 = 0x04,
    ODR_HZ4 = 0x05,
    ODR_HZ8 = 0x06,
    ODR_HZ15 = 0x07,
    ODR_HZ30 = 0x08,
}

// required by bitfield
impl From<u8> for Odr {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::ODR_POWERDOWN,
            0x01 => Self::ODR_HZ025,
            0x02 => Self::ODR_HZ050,
            0x03 => Self::ODR_HZ1,
            0x04 => Self::ODR_HZ2,
            0x05 => Self::ODR_HZ4,
            0x06 => Self::ODR_HZ8,
            0x07 => Self::ODR_HZ15,
            0x08 => Self::ODR_HZ30,
            _ => Self::ODR_HZ30, 
        }
    }
}

bitfield! {
    // STHS34PF80 CTRL1 register bits
    #[derive(Clone, Copy)]
    /// CTRL1 register bits
    pub struct CTRL1(u8);
    impl Debug;

    pub bool, bdu, set_bdu: 4;
    pub into Odr, odr, set_odr: 3,0;  // 4 bits
    // bits 7,6,5 not used
}

bitfield! {
    // STHS34PF80 FUNC_STATUS bits
    /// FUNC_STATUS register bits
    pub struct FuncStatus(u8);
    impl Debug;

    pub bool, tamb_shock_flag, set_shock_flag: 0;    // ambient temperature shock change
    pub bool, mot_flag, set_mot_flag: 1;             // motion detected
    pub bool, presence_flag, set_presence_flag: 2;   // presence detected
    // bits 7,6,5,4,3 not used
}

// required by bitfield CTRL0
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Gain {
    GainWideMode = 0x00,
    GainDefault = 0x07,
}

// required by bitfield
impl From<u8> for Gain {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::GainWideMode,
            0x07 => Self::GainDefault,
            _ => unreachable!(), 
        }
    }
}

bitfield! {
    // STHS34PF80 CTRL0 register bits
    #[derive(Clone, Copy)]
    /// CTRL0 register bits
    pub struct CTRL0(u8);
    impl Debug;
    u8;
    pub bool, bit7, _: 7;
    pub into Gain, gain, set_gain: 6,4;  // 3 bits
    // bits 3, 2 and 1 not used
    pub bool, bit0, _: 0;  // always 1 according to doc
}

/// Low pass filter divider
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
/// Low Pass Filter divider values
pub enum LPF_DIV {
    STHS34PF80_LPF_ODR_DIV_9   = 0x00,
    STHS34PF80_LPF_ODR_DIV_20  = 0x01,
    STHS34PF80_LPF_ODR_DIV_50  = 0x02,
    STHS34PF80_LPF_ODR_DIV_100 = 0x03,
    STHS34PF80_LPF_ODR_DIV_200 = 0x04,
    STHS34PF80_LPF_ODR_DIV_400 = 0x05,
    STHS34PF80_LPF_ODR_DIV_800 = 0x06,
}

impl From<u8> for LPF_DIV {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::STHS34PF80_LPF_ODR_DIV_9,
            0x01 => Self::STHS34PF80_LPF_ODR_DIV_20,
            0x02 => Self::STHS34PF80_LPF_ODR_DIV_50,
            0x03 => Self::STHS34PF80_LPF_ODR_DIV_100,
            0x04 => Self::STHS34PF80_LPF_ODR_DIV_200,
            0x05 => Self::STHS34PF80_LPF_ODR_DIV_400,
            0x06 => Self::STHS34PF80_LPF_ODR_DIV_800,
            _ => unreachable!(), 
        }
    }
}

bitfield! {
    // STHS34PF80 LPF1 register bits
    #[derive(Clone, Copy)]
    /// LPF1 register
    pub struct LPF1(u8);
    impl Debug;
    u8;
    pub into LPF_DIV, lpf_m_p, set_lpf_m_p: 5,3;  // 3 bits,  motion and presence LPF config (divider)
    pub into LPF_DIV, lpf_m, set_lpf_m: 2,0;  // 3 bits, motion detection LPF config
}


bitfield! {
    // STHS34PF80 LPF2 register bits
    #[derive(Clone, Copy)]
    /// LPF2 register
    pub struct LPF2(u8);
    impl Debug;
    u8;


    pub into LPF_DIV, lpf_p, set_lpf_p: 5,3;  // 3 bits,  presence LPF config (divider)
    pub into LPF_DIV, lpf_a_t, set_lpf_a_t: 2,0;  // 3 bits, ambient temperature shock detection LPF config
}

///----------------------------------

/// Interrupt pin configuration
#[derive(Debug)]
/// Interrupt pin configuration value
pub struct InterruptPinConfig(pub u8);

impl InterruptPinConfig {
    /// builder sets config value to 0x00
    pub fn builder() -> InterruptPinConfig {
        InterruptPinConfig(0x00)
    }
    /// gets interrupt pin config value from struct (or just just the ".0" directly)
    pub fn get_u8_value(&self) -> u8 {
        self.0
    }
    /// interrupt pin is high when active
    pub fn active_high(mut self) -> Self {
        self.0 |= 0b10000000;
        self
    }
    /// interrupt pin is low when active
    pub fn active_low(mut self) -> Self {
        self.0 &= 0b01111111;
        self
    }
    /// interrupt pin drive is push-pull
    pub fn open_drain(mut self) -> Self {
        self.0 |= 0b01000000;
        self
    }
    /// interrupt pin drive is open drain (not driven)
    pub fn push_pull(mut self) -> Self {
        self.0 &= 0b10111111;
        self
    }
    /// interrupt on new presence detcted
    pub fn on_new_presence(mut self) -> Self {
        self.0 |= 0b00100000;
        self
    }
    /// no interrupt when new presence detected
    pub fn not_new_presence(mut self) -> Self {
        self.0 &= 0b11011111;
        self
    }
    /// interrupt on new motion detected
    pub fn on_new_motion(mut self) -> Self {
        self.0 |= 0b00010000;
        self
    }
    /// no interrupt when new motion detected
    pub fn not_new_motion(mut self) -> Self {
        self.0 &= 0b11101111;
        self
    }
    /// interrupt on new shock temperature change detected
    pub fn on_new_shock_temp(mut self) -> Self {
        self.0 |= 0b00001000;
        self
    }

    /// no interrupt when new shock temperature change detected
    pub fn not_new_shock_temp(mut self) -> Self {
        self.0 &= 0b11110111;
        self
    }

    /// interrupt on new any of three changes detected
    pub fn on_new_any(mut self) -> Self {
        self.0 |= 0b00111000;
        self
    }

    /// interrupt is pulsed on pin
    pub fn pulsed_pin(mut self) -> Self {
        self.0 &= 0b11111101;
        self
    }

    /// interrupt is latched on pin
    pub fn latched_pin(mut self) -> Self {
        self.0 |= 0b00000010;
        self
    }

    /// interrupt route to high-Z  (IEN of CTRL3 to 0b00)
    pub fn ien_high_z(mut self) -> Self {
        self.0 &= 0b11111100;
        self
    }

    /// interrupt route to data ready  (IEN of CTRL3 to 0b01 == DRDY)
    pub fn ien_drdy(mut self) -> Self {
        self.0 |= 0b00000001;
        self
    }

    /// interrupt route to INT_OR  (IEN of CTRL3 to 0b10 == INT_OR)
    pub fn ien_int_or(mut self) -> Self {
        self.0 |= 0b00000010;   
        self.0 &= 0b11111011;// latch bit must be reset also for this IEN mode
        self
    }

    /// build just returns the value as u8 (to complete the builder pattern)
    pub fn build(mut self) -> u8 {
        self.0 as u8
    }


}
