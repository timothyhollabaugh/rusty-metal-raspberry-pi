
#![feature(asm, lang_items)]
#![no_std]

pub mod base;
pub mod gpio;
pub mod systimer;
pub mod aux;
pub mod tests;

// The Rust compiler expects these because there is no stdlib
#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {}


#[cfg(not(test))]
#[lang = "panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
        loop {}
}

#[export_name = "_ZN4core9panicking5panic17h35c8394187578520E"]
pub fn panic()-> ! {
     loop {}
}

// Satisfies the linker's need for _exit, _kill, etc
#[cfg(not(test))]
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}
