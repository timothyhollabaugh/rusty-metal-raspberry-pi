//! gpio
//!
//! Module for using the GPIO pins

#![allow(dead_code)]

use base;

//pub const GPIO_BASE: u32 = 0x20200000;
pub const GPIO_BASE: u32 = base::PERIPHERAL_BASE + 0x200000;

pub const GPIO: *const u32 = GPIO_BASE as *const u32;


pub const GPSEL0: isize = 0;
pub const GPSEL1: isize = 1;
pub const GPSEL2: isize = 2;
pub const GPSEL3: isize = 3;
pub const GPSEL4: isize = 4;
pub const GPSEL5: isize = 5;

pub const GPSET0: isize = 7;
pub const GPSET1: isize = 8;

pub const GPCLR0: isize = 10;
pub const GPCLR1: isize = 11;

pub const GPLEV0: isize = 13;
pub const GPLEV1: isize = 14;

pub const GPEDS0: isize = 16;
pub const GPEDS1: isize = 17;

pub const GPREN0: isize = 19;
pub const GPREN1: isize = 20;

pub const GPFEN0: isize = 22;
pub const GPFEN1: isize = 23;

/// Pin for the onboard led on the pi zero
/// Note that it is inverted on the pi zero
pub const ACT_LED: u32 = 47;

///
/// digitalWrite(pin, state)
///
/// pin: The pin to write, 0-53
/// state: The state to write. true -> high, false -> low
///
#[no_mangle]
pub fn digital_write(pin: u32, state: bool){

    let byte = if pin < 32 { 
        match state {
            true => unsafe{GPIO.offset(GPSET0) as *mut u32},
            false => unsafe{GPIO.offset(GPCLR0) as *mut u32}
        }
    } else {
        match state {
            true => unsafe{GPIO.offset(GPSET1) as *mut u32},
            false => unsafe{GPIO.offset(GPCLR1) as *mut u32}
        }
    };
        
    let bit = if pin < 32 {
        pin
    } else {
        pin - 32
    };

    unsafe {
        *(byte) |= 1 << bit
    }
}    

