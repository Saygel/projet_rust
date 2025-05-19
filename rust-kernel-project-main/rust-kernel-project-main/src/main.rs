//! Point d'entrée du noyau Rust

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::kernel_main;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main();
    loop {}
}