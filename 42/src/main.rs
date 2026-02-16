#![no_main]
#![no_std]
use core::panic::PanicInfo;

mod vga;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga::str_to_vga("Hello, world!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
