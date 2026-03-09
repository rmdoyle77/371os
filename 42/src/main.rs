#![no_main]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

mod colors;
mod vga;
mod serial;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    colors::image();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!(" [Fail]");
    serial_println!("{}", info);
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xFu32) };
    loop {}
}

fn _ex() {
}

fn _bad() {
    assert!(false);
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex, _bad, _ex];
    for i in 0..fs.len() {
        serial_print!("Beginning test 0x{:02x}...", i);
        fs[i]();
        serial_println!(" [Pass]");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) };
}
