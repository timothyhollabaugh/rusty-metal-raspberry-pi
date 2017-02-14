
#![feature(asm, lang_items)]
#![no_std]

pub mod base;
pub mod gpio;
pub mod systimer;

// The Rust compiler expects these because there is no stdlib
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}

// Satisfies the linker's need for _exit, _kill, etc
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}
