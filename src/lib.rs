#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod error;


use crate::constants::{STHS34PF80_FUNC_CFG_ADDR, STHS34PF80_FUNC_CFG_DATA, STHS34PF80_LPF1, STHS34PF80_PAGE_RW, STHS34PF80_SENS_DATA};
use crate::error::Sths34pf80Error;

pub mod data;
//use crate::data::OperationMode;

use constants::{DeviceAddress, STHS34PF80_CTRL0, STHS34PF80_HYST_MOTION, STHS34PF80_HYST_PRESENCE, STHS34PF80_HYST_TAMB_SHOCK, STHS34PF80_LPF2, STHS34PF80_MOTION_THS, STHS34PF80_PRESENCE_THS, STHS34PF80_TAMB_SHOCK_L, STHS34PF80_TAMB_SHOCK_THS, STHS34PF80_TMOTION_L, STHS34PF80_TOBJECT_L, STHS34PF80_TPRESENCE_H, STHS34PF80_TPRESENCE_L};
use data::{Gain, InterruptPinConfig, Measurements, CTRL0, LPF1, LPF2, LPF_DIV};
use num_traits::ToBytes;
use crate::data::{Avg_trim, AverageTrimTMOSFlag, AverageTrimT1Flag, FuncStatus, Odr, CTRL1};

pub mod constants;

use crate::constants::DeviceAddress::{Primary};

#[allow(unused_imports)]
use crate::constants::{STHS34PF80_PART_ID, STHS34PF80_WHO_AM_I, STHS34PF80_RESET_ALGO, STHS34PF80_CTRL3,
    STHS34PF80_CTRL2, STHS34PF80_CTRL1, STHS34PF80_AVG_TRIM, STHS34PF80_STATUS, STHS34PF80_FUNC_STATUS };

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};
// use libm::{powf, truncf};
use log::{debug, info};



/// the STHS34PF80 device
pub struct Sths34pf80<I2C, D> {
    /// I²C interface
    i2c: I2C,

    /// I²C device address
    address: u8,
    delayer: D,
}

impl<I2C, D> Sths34pf80<I2C, D>
where
    I2C: I2c<SevenBitAddress>,
    D: DelayNs,
{
    /// create new ENS160 driver with default I2C address: ADDR pin low
    pub fn new(i2c: I2C, delayer: D) -> Self {
        debug!("new called");
        Self {
            i2c,
            address: Primary.into(),
            delayer,
        }
    }

    /// give back the I2C interface
    pub fn release(self) -> I2C {
        self.i2c
    }

    // command_buf is an u8 array that starts with command byte followed by command data byte(s)
    fn write_command<const N: usize>(&mut self, command_buf: [u8; N] ) -> Result<(), Sths34pf80Error<I2C>> {
        // debug!("write_command : {:#?}", command_buf);
        self.i2c
            .write(self.address, &command_buf)
            .map_err(Sths34pf80Error::WriteError)
    }

    fn read_register( &mut self, register_address: u8, buffer: &mut [u8] ) -> Result<(), Sths34pf80Error<I2C>> {
        let mut command_buffer = [0u8; 1];
        command_buffer[0] = register_address;
        // let mut result_buffer = [0u8; N];
        self.i2c
            .write_read(self.address, &command_buffer, buffer)
            .map_err(Sths34pf80Error::WriteReadError)?;
        Ok(())
    }


    /// check if STHS34PF80 is connected
    pub fn is_connected(&mut self) -> Result<bool, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_WHO_AM_I, &mut result_buf)?;
        debug!(" is_connected() WHO_AM_I is {:#04x}", result_buf[0]);
        if (result_buf[0] == STHS34PF80_PART_ID) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// check if any data is ready
    pub fn get_data_ready(&mut self) -> Result<bool, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_STATUS, &mut result_buf)?;
        if ((result_buf[0] & 0x04) != 0x00) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// check FUNC_STATUS flags to see what data is ready 
    pub fn get_func_status(&mut self) -> Result<FuncStatus, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_FUNC_STATUS, &mut result_buf)?;
        Ok(FuncStatus(result_buf[0]))
    }

    

    /// get presence threshold
    pub fn get_presence_threshold(&mut self) -> Result<u16, Sths34pf80Error<I2C>> {
        debug!("get_presence+threahold called");
        let mut result_buf: [u8; 2] = [0; 2];
        // self.read_register(STHS34PF80_PRESENCE_THS, &mut result_buf)?;
        self.special_func_cfg_read(STHS34PF80_PRESENCE_THS, &mut result_buf)?;
        debug!("presences threshold is {:#04x} {:#04x} (LE hex)", result_buf[0], result_buf[1]);
        Ok(u16::from_le_bytes(result_buf))
    }

    
    /// get motion threshold
    pub fn get_tmotion_threshold(&mut self) -> Result<u16, Sths34pf80Error<I2C>> {
        debug!("get_tmotion_threshold called");
        let mut result_buf: [u8; 2] = [0; 2];
        // self.read_register(STHS34PF80_MOTION_THS, &mut result_buf)?;
        self.special_func_cfg_read(STHS34PF80_MOTION_THS, &mut result_buf)?;
        Ok(u16::from_le_bytes(result_buf))
    }

    /// get Tambient shock threshold
    pub fn get_tambient_shock_threshold(&mut self) -> Result<u16, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 2] = [0; 2];
        // self.read_register(STHS34PF80_TAMB_SHOCK_THS, &mut result_buf)?;
        self.special_func_cfg_read(STHS34PF80_TAMB_SHOCK_THS, &mut result_buf)?;
        Ok(u16::from_le_bytes(result_buf))
    }



    /// get shock temperature value
    pub fn get_shock_temperature(&mut self) -> Result<i16, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 2] = [0; 2];
        self.read_register(STHS34PF80_TAMB_SHOCK_L, &mut result_buf)?;
        Ok(i16::from_le_bytes(result_buf))
    }

    /// get presence value
    pub fn get_presence(&mut self) -> Result<i16, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 2] = [0; 2];
        self.read_register(STHS34PF80_TPRESENCE_L, &mut result_buf)?;
        Ok(i16::from_le_bytes(result_buf))        
    }

    /// get object temperature value 
    pub fn get_temperature(&mut self) -> Result<i16, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 2] = [0; 2];
        self.read_register(STHS34PF80_TOBJECT_L, &mut result_buf)?;
        let value: i16 =  i16::from_le_bytes(result_buf);
        Ok(value)
    }

    /// get TMOTION value
    pub fn get_tmotion(&mut self) -> Result<i16, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 2] = [0; 2];
        self.read_register(STHS34PF80_TMOTION_L, &mut result_buf)?;
        Ok(i16::from_le_bytes(result_buf))
    }


     /// read func_cfg_register special, one byte at a time because the STHS34PF80_FUNC_CFG_ADDR 
     /// note that auto-increment does not work on read. 
     pub fn special_func_cfg_read<const N: usize>(&mut self, register_address: u8, buffer: &mut [u8; N] ) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("in func_cfg_read for register address : {:#04x}", register_address);
        let mut command_buffer = [0u8; 1];
        command_buffer[0] = register_address;
        // save current odr and power down bit in ctrl1
        let mut result_buf:[u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let current_ctrl1: CTRL1 = CTRL1(result_buf[0]);
        let current_odr: Odr = current_ctrl1.odr();
        
        // enable access to embedded function registers
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        let mut current_ctrl2 = result_buf[0];  
        current_ctrl2 = current_ctrl2 | 0x10;  // bit 4 is FUNC_CFG_ACCESS
        self.write_command([STHS34PF80_CTRL2, current_ctrl2])?;
        debug!(" wrote ctrl2 = {:#04x} to enable access to FUNC_CFG registers", current_ctrl2);

        // enable read mode
        self.write_command([STHS34PF80_PAGE_RW, 0x20])?;   // set bit 5 to 1

        // select the embedded func register address
        //self.write_command([STHS34PF80_FUNC_CFG_ADDR, register_address])?;
        let mut new_buffer_2: [u8; 1] = [0; 1];
        //self.read_register(STHS34PF80_FUNC_CFG_DATA, &mut new_buffer_2)?;
        let mut index: usize = 0;
        for i in 0..N {
            // select the embedded func byte register address
            index = register_address as usize +i;
            self.write_command([STHS34PF80_FUNC_CFG_ADDR, index as u8])?;
            // read the register byte
            self.read_register(STHS34PF80_FUNC_CFG_DATA, &mut new_buffer_2)?;
            buffer[i] = new_buffer_2[0];
        }

        /***
        buffer[0] = new_buffer_2[0];
         if (N == 2) {
            self.write_command([STHS34PF80_FUNC_CFG_ADDR, register_address+1])?;
            self.read_register(STHS34PF80_FUNC_CFG_DATA, &mut new_buffer_2)?;
            buffer[1] = new_buffer_2[0];
         }
         ****/
        /* ****
                // read the register
        command_buffer = [STHS34PF80_FUNC_CFG_DATA];
        let mut new_buffer: [u8; 2] = [0; 2];
        self.read_register(STHS34PF80_FUNC_CFG_DATA, &mut new_buffer)?;

        debug!("   new_buffer[0] = {:#04x}, new_buffer[1] = {:#04x}", new_buffer[0], new_buffer[1]);
        for n in 0..2 {
            buffer[n] = new_buffer[n];
        }
        ****/

        // disable read mode 
        self.write_command([STHS34PF80_PAGE_RW, 0x00])?;   // set bit 5 to 0 

        // disable access to embedded functions register *
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        current_ctrl2 = result_buf[0];  
        current_ctrl2 = current_ctrl2 & 0xef;  // bit 4 is FUNC_CFG_ACCESS
        self.write_command([STHS34PF80_CTRL2, current_ctrl2])?;
        debug!(" wrote ctrl2 = {:#04x} to disable access to FUNC_CFG registers", current_ctrl2);


        // set saved odr back 
        self.tmos_odr_check_safe_set(current_ctrl1, current_odr)?;
                
        Ok(())

    }


    pub fn func_cfg_write(&mut self, regsiter_address: u8, data_1: u8, data_2: u8, len: i16) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("func_cfg_write called, addr = {:#04x}, data_1 = {:#04x}, data_2 = {:#04x}", regsiter_address,
                data_1, data_2);
        // save current ctrl1 and odr value
        let mut result_buf:[u8;1] = [0;1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let current_ctrl1 = CTRL1(result_buf[0]);
        let current_odr: Odr = current_ctrl1.odr();

        // go to power down mode
        self.tmos_odr_check_safe_set(current_ctrl1, Odr::ODR_POWERDOWN)?;

        // enable access to embedded function registers
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        let mut current_ctrl2 = result_buf[0];  
        current_ctrl2 = current_ctrl2 | 0x10;  // bit 4 is FUNC_CFG_ACCESS
        self.write_command([STHS34PF80_CTRL2, current_ctrl2])?;
        debug!(" wrote ctrl2 = {:#04x} to enable access to FUNC_CFG registers", current_ctrl2);
                
        // enable write mode
        self.write_command([STHS34PF80_PAGE_RW, 0x40])?;   // set bit 6

        // select the embedded func register address (it will autoincrement when writing) 
        self.write_command([STHS34PF80_FUNC_CFG_ADDR, regsiter_address])?;

        // write the data to selected address  (no vector in no_std so this kind-a sucks)
        if (len == 1) {
            self.write_command([STHS34PF80_FUNC_CFG_DATA, data_1])?;
        } else if (len == 2) {
            self.write_command([STHS34PF80_FUNC_CFG_DATA, data_1])?;
            self.write_command([STHS34PF80_FUNC_CFG_DATA, data_2])?;
            debug!("     wrote data_1 = {:#04x} data_2 = {:#04x}", data_1, data_2);
        }

        debug!("  wrote data byte(s) to func cfg register {:#04x}", regsiter_address);
        // disable write mode
        self.write_command([STHS34PF80_PAGE_RW, 0x00])?;

        // disable access to func_cfg registers
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        current_ctrl2 = result_buf[0];  // bit 4 is FUNC_CFG_ACCESS
        current_ctrl2 = current_ctrl2 & 0xef;
        self.write_command([STHS34PF80_CTRL2, current_ctrl2])?;

        // set saved odr back saved value */
        self.tmos_odr_check_safe_set(current_ctrl1, current_odr)?;
        debug!("  wrote saved ctrl1 back, {:#04x}", current_ctrl1.0);
        Ok(())

    }

    // rename this fn
    fn tmos_odr_check_safe_set(&mut self, new_ctrl1: CTRL1, new_odr: Odr) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("tmos_odr_check_safe_set called");
        let mut ctrl1: CTRL1 = new_ctrl1.clone();
        if (new_odr != Odr::ODR_POWERDOWN) {
            // need to reset the algorithm when odr is changed from continuous (Hz > 0)
            //
            debug!("  new_odr is not ODR_POWERDOWN"); 
            ctrl1.set_odr(Odr::ODR_POWERDOWN as u8);
            self.write_command([STHS34PF80_CTRL1, ctrl1.0])?;
            self.algo_reset()?;
        } else { // going to power down mode, must not be performing a new measurement
            // reset data_ready by reading FUNC_STATUS register
            debug!("  new_odr is ODR_POWERDOWN");
            self.get_func_status().unwrap();
            // wait for data_ready to be set
            loop {
                if let Ok(data_ready) = self.get_data_ready() {
                  break;
                } else {
                    self.delayer.delay_ms(2);
                } 
            }
            // can now go to power down mode
            debug!("  going to power down mode now");
            ctrl1.set_odr(Odr::ODR_POWERDOWN as u8);
            self.write_command([STHS34PF80_CTRL1, ctrl1.0])?;

            // reset data_ready again
            self.get_func_status().unwrap();
        }
        ctrl1.set_odr(new_odr as u8);
        self.write_command([STHS34PF80_CTRL1, ctrl1.0])?;
        debug!("  done.");
        Ok(())

    }

     /// algorithm reset
     pub fn algo_reset(&mut self) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("  in algo_reset");
        self.func_cfg_write(STHS34PF80_RESET_ALGO, 0x01, 0x00, 1)?;
        debug!("  algo_reset done");
        Ok(())
    }

    /// reset STHS34PF80 to standard mode, 32 samples, 1 Hz continuous op mode
    pub fn reset_device(&mut self) -> Result<bool, Sths34pf80Error<I2C>> {
        Ok((false))
    }

    pub fn initialize(&mut self) -> Result<(), Sths34pf80Error<I2C>> {
        debug!(" better initialize called");
        if (!self.is_connected()?) {
            return Err(Sths34pf80Error::NotConnected);
            // Sths34pf80Error::NotConnected;  // although this will not occur as above is either true or error
        }
        debug!("  is_connected is true");
        let mut result_buf = [0; 1];

        // just read the CTRL2 regsiter first to get a response or error
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        self.write_command([STHS34PF80_CTRL2, 0x80])?; // bit 7 is boot bit
        debug!("  wrote boot bit to CTRL2");
        self.delayer.delay_ms(5);
        self.algo_reset()?;

        self.set_avg_tmos_num(AverageTrimTMOSFlag::AVG_TMOS32)?;
        debug!("  wrote AVG_TMOS32 to avg_tmos_trim register");

        self.set_avg_t1_trim(AverageTrimT1Flag::AVG_T8)?;
        debug!("  wrote AVG_t8 to avg_t1_trim register");

        self.set_block_data_update(true)?;
        debug!("  set block data update flag to true");

        // set the data rate (ODR) to 1Hz
        self.set_tmos_odr(Odr::ODR_HZ1)?;
        debug!("  set ODR to 1 Hz");
        
        Ok(())
    }


    /// set avgerage tmos trim
    pub fn set_avg_tmos_num(&mut self, avg_tmos_trim: AverageTrimTMOSFlag) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("in set_avg_tmos_num");
        let mut result_buf:[ u8; 1] = [0;1];
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?;  // get the register value
        let mut avg_trim: Avg_trim = Avg_trim(result_buf[0]);
        // because it includes both ambient temperature and object temperature number of samples
        avg_trim.set_trim_t1(avg_tmos_trim as u8);
        self.write_command([STHS34PF80_AVG_TRIM, avg_trim.0])?;
        Ok(())
    }

    /// set ambient temperature average trim
    pub fn set_avg_t1_trim(&mut self, avg_t1_trim: AverageTrimT1Flag) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("in set_avg_t1_trim");
        let mut result_buf: [u8;1] = [0;1];
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?; 
        let mut avg_trim: Avg_trim = Avg_trim(result_buf[0]);
        avg_trim.set_trim_t1(avg_t1_trim as u8);
        debug!("  writing AVG_TRIM to {:#04x}", avg_trim.0);
        self.write_command([STHS34PF80_AVG_TRIM, avg_trim.0])?;
        Ok(()) 
    }

    /// set block data rate update flag
    pub fn set_block_data_update(&mut self, new_bdu: bool) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("in set_block_data_update");
        let mut result_buf: [u8; 1] = [0;1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        debug!(" read_register(CTRL1) returned {:#04x}", result_buf[0]);
        let mut ctrl1_value = CTRL1(result_buf[0]);
        debug!(" initial CTRL1 value is {:#04x}", ctrl1_value.0);
        ctrl1_value.set_bdu(new_bdu);  // allow ODR to be changed
        debug!("  writing ctrl1 value (bdu bit 4) {:#04x}", ctrl1_value.0);
        self.write_command([STHS34PF80_CTRL1, ctrl1_value.0])?;
        Ok(())
    }

    /// get tmos ODR (Output Data Rate)
    pub fn get_tmos_odr(&mut self) -> Result<Odr, Sths34pf80Error<I2C>> {
        // debug!{"in get_tmos_odr"};
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let ctrl1: CTRL1 = CTRL1(result_buf[0]);
        let odr = ctrl1.odr();
        // debug!("  ctrl1 is {:#04x} and odr is {:#?}", ctrl1.0, odr);
        Ok(odr)
    }

    /// set tmos odr bits
    pub fn set_tmos_odr(&mut self, odr_new: Odr) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("in set_tmos_odr");
        let mut result_buf: [u8;1] = [0; 1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let ctrl1: CTRL1 = CTRL1(result_buf[0]);
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?;
        let avg_trim: Avg_trim = Avg_trim(result_buf[0]);

        // check Odr limits by current avg_tmos setting
        let mut odr_max: Odr = Odr::ODR_HZ1;
        match avg_trim.average_trim_tmos_flag() {
            AverageTrimTMOSFlag::AVG_TMOS2 => odr_max = Odr::ODR_HZ30,
            AverageTrimTMOSFlag::AVG_TMOS8 => odr_max = Odr::ODR_HZ30,
            AverageTrimTMOSFlag::AVG_TMOS32 => odr_max = Odr::ODR_HZ30,
            AverageTrimTMOSFlag::AVG_TMOS128 => odr_max = Odr::ODR_HZ8,
            AverageTrimTMOSFlag::AVG_TMOS256 => odr_max = Odr::ODR_HZ4,
            AverageTrimTMOSFlag::AVG_TMOS512 => odr_max = Odr::ODR_HZ2,
            AverageTrimTMOSFlag::AVG_TMOS1024 => odr_max = Odr::ODR_HZ1,
            AverageTrimTMOSFlag::AVG_TMOS2048 => odr_max = Odr::ODR_HZ050,
        };
        if (odr_new > odr_max) {
            debug!("  odr_new in too big");
            return Err(Sths34pf80Error::OdrNewTooBig(odr_max as u8));
        }
        debug!("  odr_new is good, setting it now");
        self.tmos_odr_check_safe_set(ctrl1, odr_new)?;
        Ok(())
    } 


    /// set presence threshold as 15 bit u16 (i.e 0 < value <= 0x7fff)
    pub fn set_presence_threshold_new(&mut self, value_new: u16) -> Result<bool, Sths34pf80Error<I2C>> {
        debug!("in set_presence_threshold_new {}", value_new);
        // *** bug in STHS34PF80: it only reads the low byte value.
        if value_new > 0x7fff {
            return Err(Sths34pf80Error::ThresholdTooBig(value_new));
        }
        // not sure what happened if value_new is == 0, nothing in doc about that
        // save current odr and power_down bits in CTRL1
        
        // ToDo: just testing below
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let current_ctrl1: CTRL1 = CTRL1(result_buf[0]);
        let current_odr = current_ctrl1.odr();
        let current_pd: bool = current_ctrl1.bdu();
        debug!("  *** current ctrl1 is {:?} and odr is {:?}", current_ctrl1, current_odr);

        // go to power down mode
        // no, done in func_cfg_write() self.tmos_odr_check_safe_set(current_ctrl1,Odr::ODR_POWERDOWN)?;

        let write_buf1: u8 = value_new.to_le_bytes()[0];
        let write_buf2: u8 = value_new.to_le_bytes()[1];
        // debug!("  write_buf1 : {}, write_buf2 : {}", write_buf1, write_buf2);

        // write the new presence threshold value to func_cfg_register
        
        self.func_cfg_write(STHS34PF80_PRESENCE_THS, write_buf1, write_buf2, 2)?;
        //self.func_cfg_write(STHS34PF80_PRESENCE_THS, write_buf1, 0x00, 1)?;
        //self.func_cfg_write(STHS34PF80_PRESENCE_THS+1, write_buf2, 0x00, 1)?;
        debug!("  *** algo_reset ***");
        self.algo_reset()?;
        // restore odr value in CTRL1
        debug!("  *** tmos_odr_check_safe_set to {:?}", current_odr);
        self.tmos_odr_check_safe_set(current_ctrl1, current_odr)?;
        debug!("  *** done set_presence_threshold_new");
        Ok(true)

    }


    /// set tmotion threshold as 15 bit u16 (i.e 0 < value <= 0x7fff)
    pub fn set_tmotion_threshold_new(&mut self, value_new: u16) -> Result<bool, Sths34pf80Error<I2C>> {
        debug!("in set_tmotion_threshold_new {}", value_new);
        if value_new > 0x7fff {
            return Err(Sths34pf80Error::ThresholdTooBig(value_new));
        }
        // not sure what happened if value_new is == 0, nothing in doc about that
        // save current odr and power_down bits in CTRL1
        
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let current_ctrl1: CTRL1 = CTRL1(result_buf[0]);
        let current_odr = current_ctrl1.odr();
        let current_pd: bool = current_ctrl1.bdu();
        //debug!("  *** current ctrl1 is {:?} and odr is {:?}", current_ctrl1, current_odr);

        // go to power down mode
        // no, done in func_cfg_write() self.tmos_odr_check_safe_set(current_ctrl1,Odr::ODR_POWERDOWN)?;

        let write_buf1: u8 = value_new.to_le_bytes()[0];
        let write_buf2: u8 = value_new.to_le_bytes()[1];

        // write the new presence threshold value to func_cfg_register
        //debug!("  *** func_cfg_write ***");
        self.func_cfg_write(STHS34PF80_MOTION_THS, write_buf1, write_buf2, 2)?;
        //self.func_cfg_write(STHS34PF80_PRESENCE_THS, write_buf1, 0x00, 1)?;
        //self.func_cfg_write(STHS34PF80_PRESENCE_THS+1, write_buf2, 0x00, 1)?;
        //debug!("  *** algo_reset ***");
        self.algo_reset()?;
        // restore odr value in CTRL1
        debug!("  *** tmos_odr_check_safe_set to {:?}", current_odr);
        self.tmos_odr_check_safe_set(current_ctrl1, current_odr)?;
        //debug!("  *** done set_tmotion_threshold_new");
        Ok(true)
    }

    /// set tambient shock threshold as 15 bit u16 (i.e 0 < value <= 0x7fff)
    pub fn set_tambient_shock_threshold_new(&mut self, value_new: u16) -> Result<bool, Sths34pf80Error<I2C>> {
        debug!("in est_tambient_shock_threshold_new {}", value_new);
        if value_new > 0x7fff {
            return Err(Sths34pf80Error::ThresholdTooBig(value_new));
        }
        // not sure what happened if value_new is == 0, nothing in doc about that
        // save current odr and power_down bits in CTRL1
        
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL1, &mut result_buf)?;
        let current_ctrl1: CTRL1 = CTRL1(result_buf[0]);
        let current_odr = current_ctrl1.odr();
        let current_pd: bool = current_ctrl1.bdu();
        //debug!("  *** current ctrl1 is {:?} and odr is {:?}", current_ctrl1, current_odr);

        // go to power down mode
        // no, done in func_cfg_write() self.tmos_odr_check_safe_set(current_ctrl1,Odr::ODR_POWERDOWN)?;

        let write_buf1: u8 = value_new.to_le_bytes()[0];
        let write_buf2: u8 = value_new.to_le_bytes()[1];

        // write the new presence threshold value to func_cfg_register
        //debug!("  *** func_cfg_write ***");
        self.func_cfg_write(STHS34PF80_TAMB_SHOCK_THS, write_buf1, write_buf2, 2)?;
        //self.func_cfg_write(STHS34PF80_PRESENCE_THS, write_buf1, 0x00, 1)?;
        //self.func_cfg_write(STHS34PF80_PRESENCE_THS+1, write_buf2, 0x00, 1)?;
        //debug!("  *** algo_reset ***");
        self.algo_reset()?;
        // restore odr value in CTRL1
        //debug!("  *** tmos_odr_check_safe_set to {:?}", current_odr);
        self.tmos_odr_check_safe_set(current_ctrl1, current_odr)?;
        //debug!("  *** done set_tambient_shock_threshold_new");
        Ok(true)
    }

    /// set number of averages for object temperature
    // ToDo: test this
    pub fn set_avg_tobject_num(&mut self, avg_tmos_trim: AverageTrimTMOSFlag) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?;
        let mut avg_trim_current: Avg_trim = Avg_trim(result_buf[0]);
        avg_trim_current.set_trim_tmos(avg_tmos_trim as u8);
        self.write_command([STHS34PF80_AVG_TRIM, avg_trim_current.0])?;
        Ok(())
    }

    /// get number of averages for object temperature
    // ToDo:  test this
    pub fn get_avg_tobject_num(&mut self) -> Result<AverageTrimTMOSFlag, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?;
        let avg_trim_current: Avg_trim = Avg_trim(result_buf[0]);
        let avg_trim_tmos_flag: AverageTrimTMOSFlag = avg_trim_current.average_trim_tmos_flag();
        Ok(avg_trim_tmos_flag)
    }

    /// set number of averages for ambient temperature
    pub fn set_avg_tambient_num(&mut self, avg_t1_trim: AverageTrimT1Flag) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?;
        let mut avg_trim_current: Avg_trim = Avg_trim(result_buf[0]);
        avg_trim_current.set_trim_t1(avg_t1_trim as u8);
        self.write_command([STHS34PF80_AVG_TRIM, avg_trim_current.0])?;
        Ok(())
    }

    /// get number of averages for ambient temperature
    pub fn get_avg_tambient_num(&mut self) -> Result<AverageTrimT1Flag, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_AVG_TRIM, &mut result_buf)?;
        debug!(" get_avg_tambient_num read raw {:#04x}", result_buf[0]);
        let avg_trim_current: Avg_trim = Avg_trim(result_buf[0]);
        let avg_trim_t1: AverageTrimT1Flag = avg_trim_current.average_trim_t1_flags();
        Ok(avg_trim_t1)
    }


    /// set_gain_mode in CTRL0, either default or wide mode
    pub fn set_gain_mode(&mut self, gain_mode_new: Gain) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL0, &mut result_buf)?;
        let mut ctrl0_current = CTRL0(result_buf[0]);
        ctrl0_current.set_gain(gain_mode_new as u8);
        self.write_command([STHS34PF80_CTRL0, ctrl0_current.0])?;
        Ok(())
    }

    /// get gain_mode from CTRL0 register
    pub fn get_gain_mode(&mut self) -> Result<Gain, Sths34pf80Error<I2C>> {
        debug!("in get_gain_mode");
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL0, &mut result_buf)?;
        let ctrl0: CTRL0 = CTRL0(result_buf[0]);
        let gain: Gain = ctrl0.gain();
        // debug!(" gain is {:#?} and in hex {}", gain, gain as u8);
        Ok(ctrl0.gain())
    }

    /// get TMOS sensitivity; mkae sure to save the factory value before changing just in case...
    pub fn get_tmos_sensitivty(&mut self) -> Result<f32, Sths34pf80Error<I2C>> {
        debug!("in get_tmos_sensivity");
        let mut result_buf: [u8; 1] = [0; 1];
        
        self.read_register(STHS34PF80_SENS_DATA, &mut result_buf)?;
        let mut unscaled_value:i8 = result_buf[0] as i8;
        //let mut value: f32 = (result_buf[0] as i8) as f32;
        debug!("  raw value read is : {:#04x}", result_buf[0]);
        debug!("  unscaled_value:i8 is {}", unscaled_value);
        // scale as * 16 + 2048
        let value = unscaled_value as f32 * 16.0 + 2048.0;
        Ok(value)
    }

    /// set TMOS sensivity  :  only if external len is added
    /// acceptable range is 0.0 -to +4080.0
    pub fn set_tmos_sensitivity(&mut self, new_value: f32) -> Result<(), Sths34pf80Error<I2C>> {
        debug!("in set_tmos_sensivity to {}", new_value);
        // value is = (new_value - 2048) / 16
        // it must be a valid i8 (signed 2's comp) to be written
        let mut write_value: f32 = (new_value - 2048.0) / 16.0;
        debug!(" scaled write value is {}", write_value);
        if ( (write_value < -128.0) || (write_value > 127.0) ) {
            return Err(Sths34pf80Error::OutOfRange());
        }
        let mut write_u8:u8 = ((write_value as i16) & 0x00ff) as u8;
        debug!("  writing new value of {:#04x}", write_u8);
        // NOT YETself.write_command([STHS34PF80_SENS_DATA, write_value as u8])?;
        Ok(())
    }

    /// get_config_interrupt_pin value (CTRL3)
    pub fn get_config_interrupt_pin(&mut self) -> Result<InterruptPinConfig, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL3, &mut result_buf)?;
        Ok(InterruptPinConfig(result_buf[0]))
    }

    /// configure the interrupt pin of STHS34PF80.  See data sheet section 10.9 for config:u8 parameter
    /// or use the handy InterruptPinConfig::builder() and its function to generate the
    /// config:u8 parameter for you.
    /// returns the interrupt config register (CTRL3) read back (should equal value written)
    pub fn set_config_interrupt_pin(&mut self, config: u8) -> Result<InterruptPinConfig, Sths34pf80Error<I2C>> {
        self.write_command([STHS34PF80_CTRL3, config])?;
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL3, &mut result_buf)?;
        Ok(InterruptPinConfig(result_buf[0]))
    }

    /// get object temperature (IR) output register (raw temperature) in degrees C
    pub fn get_tobject_raw_in_c(&mut self) -> Result<f32, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 2] = [0; 2];
        self.read_register(STHS34PF80_TOBJECT_L, &mut result_buf)?;
        let unscaled: i16 = i16::from_le_bytes(result_buf);
        Ok((unscaled as f32) / 2000.0)  // scale to degree C according to datasheet
    }

    // *********** ToDo: test all below *********************************************************
    /// get tmos one shot trigger  (true == ONE SHOT trigger set, false == idle)
    pub fn get_tmos_one_shot_trigger(&mut self) -> Result<bool, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        if ((result_buf[0] & 0x01) != 0x00) {
            return Ok(true)
        } else {
            return Ok(false)
        }
    }

    /// set tmos one shot trigger or idle (trigger: true = set one shot trigger, false = idle)
    pub fn set_tmos_one_shot_trigger(&mut self, trigger: bool) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_CTRL2, &mut result_buf)?;
        let mut new_value: u8 = 0x00;
        if (trigger) {
            new_value = result_buf[0] | 0x01;  // bit 0 set for one shot acquisition trigger
        } else {
            new_value = result_buf[0] & 0xfe;  // bit 0 reset for idle mode
        }
        self.write_command([STHS34PF80_CTRL2, new_value])?;
        Ok(()) 
    }

    /// get motion hysteresis
    pub fn get_motion_hysteresis(&mut self) -> Result<u8, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.special_func_cfg_read(STHS34PF80_HYST_MOTION, &mut result_buf)?;
        Ok(result_buf[0])
    }

    /// set motion hysteresis
    pub fn set_motion_hysteresis(&mut self, new_value: u8) -> Result<(), Sths34pf80Error<I2C>> {
        self.func_cfg_write(STHS34PF80_HYST_MOTION, new_value, 0x00, 1)?;
        Ok(())
    }

     /// get presence hysteresis
     pub fn get_presence_hysteresis(&mut self) -> Result<u8, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.special_func_cfg_read(STHS34PF80_HYST_PRESENCE, &mut result_buf)?;
        Ok(result_buf[0])
    }

    /// set presence hysteresis
    pub fn set_preesence_hysteresis(&mut self, new_value: u8) -> Result<(), Sths34pf80Error<I2C>> {
        self.func_cfg_write(STHS34PF80_HYST_PRESENCE, new_value, 0x00, 1)?;
        Ok(())
    }

    /// get tamb_shock hysteresis
    pub fn get_tamb_shock_hysteresis(&mut self) -> Result<u8, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.special_func_cfg_read(STHS34PF80_HYST_TAMB_SHOCK, &mut result_buf)?;
        Ok(result_buf[0])
    }

    /// set tamb_shock hysteresis
    pub fn set_tamb_shock_hysteresis(&mut self, new_value: u8) -> Result<(), Sths34pf80Error<I2C>> {
        self.func_cfg_write(STHS34PF80_HYST_TAMB_SHOCK, new_value, 0x00, 1)?;
        Ok(())
    }



    // ToDo:  
    //  a. LP Filter parameters setters and getters

    /// get LPF config for motion
    pub fn get_lpf_m_bandwidth(&mut self) -> Result<LPF_DIV, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF1, &mut result_buf)?;
        let lpf1: LPF1 = LPF1(result_buf[0]);
        Ok(lpf1.lpf_m())
    }

    /// set LPF config for motion
    pub fn set_lpf_m_bandwidth(&mut self, lpf_div: LPF_DIV) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF1, &mut result_buf)?;
        let mut lpf1: LPF1 = LPF1(result_buf[0]);
        lpf1.set_lpf_m(lpf_div as u8);
        self.write_command([STHS34PF80_LPF1, lpf1.0])?;
        Ok(())
    }

    /// get LPF presence and motion config
    pub fn get_lpf_p_m_bandwidth(&mut self) -> Result<LPF_DIV, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF1, &mut result_buf)?;
        let lpf1: LPF1 = LPF1(result_buf[0]);
        Ok(lpf1.lpf_m_p())
    }

    /// set LPF presence and motion config
    pub fn set_lpf_p_m_bandwidth(&mut self, lpf_div: LPF_DIV) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF1, &mut result_buf)?;
        let mut lpf1: LPF1 = LPF1(result_buf[0]);
        lpf1.set_lpf_m_p(lpf_div as u8);
        self.write_command([STHS34PF80_LPF1, lpf1.0])?;
        Ok(())
    }

    /// get LPF2 ambient temperature shock config
    pub fn get_lpf_a_t_bandwidth(&mut self) -> Result<LPF_DIV, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF2, &mut result_buf)?;
        let lpf2: LPF2 = LPF2(result_buf[0]);
        Ok(lpf2.lpf_a_t())
    }

    /// set LPF2 ambient temperature shock config
    pub fn set_lpf_a_t_bandwidth(&mut self, lpf_div: LPF_DIV) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF2, &mut result_buf)?;
        let mut lpf2: LPF2 = LPF2(result_buf[0]);
        lpf2.set_lpf_a_t(lpf_div as u8);
        self.write_command([STHS34PF80_LPF2, lpf2.0])?;
        Ok(())
    }

    /// get LPF2 presence config
    pub fn get_lpf_p_bandwidth(&mut self) -> Result<LPF_DIV, Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF2, &mut result_buf)?;
        let lpf2: LPF2 = LPF2(result_buf[0]);
        Ok(lpf2.lpf_p())
    }

    /// set LPF2 presence config
    pub fn set_lpf_p_bandwidth(&mut self, lpf_div: LPF_DIV) -> Result<(), Sths34pf80Error<I2C>> {
        let mut result_buf: [u8; 1] = [0; 1];
        self.read_register(STHS34PF80_LPF2, &mut result_buf)?;
        let mut lpf2: LPF2 = LPF2(result_buf[0]);
        lpf2.set_lpf_p(lpf_div as u8);
        self.write_command([STHS34PF80_LPF2, lpf2.0])?;
        Ok(())
    }


    /// get measurements : blocks on data ready and func_status ready or forever (whichever comes first)
    pub fn get_measurements_blocking(&mut self) -> Result<Measurements, Sths34pf80Error<I2C>> {
        debug!("in get_measurements_blocking");
        let mut forever = true;
        let mut measurements: Measurements = Measurements{presence_value: None,
                                                         motion_value: None,
                                                         ambient_shock_value: None};
        while (forever) {
            if let Ok(data_ready) = self.get_data_ready() {
                if (data_ready) {
                    // debug!("  data_ready is true");  
                    if let Ok(func_status) = self.get_func_status() {
                       //info!("sth34pf80 data ready, func status is {:#?}", func_status);
                        if (func_status.presence_flag()) {
                            let presence_val = self.get_presence()?;
                            measurements.presence_value = Some(presence_val);
                            debug!(" presence detected! presence value is {} /cm", presence_val);
                        }
                        if (func_status.mot_flag()) {
                            let tmotion_val = self.get_tmotion()?;
                            measurements.motion_value = Some(tmotion_val);
                            debug!(" motion detected!  tmotion value is {} C ", tmotion_val);
                        }
                        if (func_status.tamb_shock_flag()) {
                            let tambient_shock_val = self.get_shock_temperature()?;
                            measurements.ambient_shock_value = Some(tambient_shock_val);
                            debug!("  ambient shock detected! tambient shock temperature is {} C", tambient_shock_val);
                        }
                        if (func_status.presence_flag() || func_status.mot_flag() || func_status.tamb_shock_flag()) {
                            debug!("  Ok with measurements filled {:#?}", measurements);
                            return Ok(measurements);
                        }
                 } else {
                    //debug!("  func_status not available");
                 }
        
                // info!("get_tobject_raw_in_c is {}", sths34pf80.get_tobject_raw_in_c().unwrap());
                
                } else {  // loop until data is ready or forever
                    // info!("sth34pf80 data not ready");
                }
            }
            // debug!("  delay_ms(500");
            self.delayer.delay_ms(50);
        }
        Ok(measurements)
    }


    /// get measurements with timeout.  timeout_ms50 is u16 in 50 msec units
    pub fn get_measurements_timeout(&mut self, timeouts_50ms: u16) -> Result<Measurements, Sths34pf80Error<I2C>> {
        debug!("in get_measurements_blocking");
        let mut timeout = false;
        let mut loop_count: u16 = timeouts_50ms;
        let mut measurements: Measurements = Measurements{presence_value: None,
                                                         motion_value: None,
                                                         ambient_shock_value: None};
        while (!timeout) {
            if let Ok(data_ready) = self.get_data_ready() {
                if (data_ready) {
                    // debug!("  data_ready is true");  
                    if let Ok(func_status) = self.get_func_status() {
                       //info!("sth34pf80 data ready, func status is {:#?}", func_status);
                        if (func_status.presence_flag()) {
                            let presence_val = self.get_presence()?;
                            measurements.presence_value = Some(presence_val);
                            debug!(" presence detected! presence value is {} /cm", presence_val);
                        }
                        if (func_status.mot_flag()) {
                            let tmotion_val = self.get_tmotion()?;
                            measurements.motion_value = Some(tmotion_val);
                            debug!(" motion detected!  tmotion value is {} C ", tmotion_val);
                        }
                        if (func_status.tamb_shock_flag()) {
                            let tambient_shock_val = self.get_shock_temperature()?;
                            measurements.ambient_shock_value = Some(tambient_shock_val);
                            debug!("  ambient shock detected! tambient shock temperature is {} C", tambient_shock_val);
                        }
                        if (func_status.presence_flag() || func_status.mot_flag() || func_status.tamb_shock_flag()) {
                            debug!("  Ok with measurements filled {:#?}", measurements);
                            return Ok(measurements);
                        }
                 } else {
                    self.delayer.delay_ms(50);
            	    loop_count += 1;
            	    if (loop_count >= timeouts_50ms) {
           	            timeout = true;
           	            return Err(Sths34pf80Error::MeasurementTimeout());
		            }
                 }
        
                // info!("get_tobject_raw_in_c is {}", sths34pf80.get_tobject_raw_in_c().unwrap());
                
                } else {  // loop until data is ready or timeout
                    self.delayer.delay_ms(50);
                    loop_count += 1;
                    if (loop_count >= timeouts_50ms) {
                       timeout = true;
                       return Err(Sths34pf80Error::MeasurementTimeout());
                    }
                }
            } else {
                timeout = true;
                return Err(Sths34pf80Error::MeasurementTimeout());
            }
		}	                   
        Ok(measurements)
    }


}
