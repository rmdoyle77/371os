#![no_main]
#![no_std]
use core::panic::PanicInfo;

mod colors;
mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    colors::image();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
