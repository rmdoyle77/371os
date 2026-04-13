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
    osirs::init();
    println!("Enter time (HH MM SS): ");
    let mut clock_started = false;
let mut last_secs = usize::MAX;
loop {
    let running = osirs::interrupts::CLOCK_RUNNING.load(core::sync::atomic::Ordering::Relaxed);
    if running {
        if !clock_started {
            clock_started = true;
            osirs::vga::clear();
        }
        let ticks = osirs::interrupts::TICKS.load(core::sync::atomic::Ordering::Relaxed);
        let start = osirs::interrupts::START_TICKS.load(core::sync::atomic::Ordering::Relaxed);
        let digits = *osirs::interrupts::INPUT_DIGITS.lock();
        let elapsed_secs = (ticks - start) / 18;
        if elapsed_secs != last_secs {
            last_secs = elapsed_secs;
            let h = digits[0] as usize * 10 + digits[1] as usize;
            let m = digits[2] as usize * 10 + digits[3] as usize;
            let s = digits[4] as usize * 10 + digits[5] as usize;
            let total = h * 3600 + m * 60 + s + elapsed_secs;
            let hh = (total / 3600) % 24;
            let mm = (total % 3600) / 60;
            let ss = total % 60;
            let mut buf = [0u8; 8];
            buf[0] = b'0' + (hh / 10) as u8;
            buf[1] = b'0' + (hh % 10) as u8;
            buf[2] = b':';
            buf[3] = b'0' + (mm / 10) as u8;
            buf[4] = b'0' + (mm % 10) as u8;
            buf[5] = b':';
            buf[6] = b'0' + (ss / 10) as u8;
            buf[7] = b'0' + (ss % 10) as u8;
            let saved = osirs::vga::get_latest();
            osirs::vga::write_at(80 * 24 + 72, core::str::from_utf8(&buf).unwrap());
            osirs::vga::set_latest(saved);
        }
    }
    x86_64::instructions::hlt();
}
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
    assert!(1 + 1 == 2);
}

fn _bad() {
    assert!(false);
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex, _ex];
    for i in 0..fs.len() {
        serial_print!("Beginning test 0x{:02x}...", i);
        fs[i]();
        serial_println!(" [Pass]");
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) };
}
