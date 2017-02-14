//! base.rs

pub const PERIPHERAL_BASE: u32 = 0x20000000;

pub fn nothing() {
    unsafe { asm!(""); }
}
