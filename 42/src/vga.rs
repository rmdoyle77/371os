static mut LATEST: usize = 0;
const MMIO: *mut u8 = 0xb8000 as *mut u8;
const COLOR: u8 = 0xF;

pub struct Dummy { } 

impl core::fmt::Write for Dummy {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        str_to_vga(s); 
        return Ok(());
    }
}

fn char_to_vga(a: u8) {
    unsafe {
        let rel: *mut u8 = ((MMIO as usize) + (LATEST * 2)) as *mut u8;
        *rel = a;
        *((rel as usize + 1) as *mut u8) = COLOR;
        LATEST = LATEST + 1;
    }
}

const ROWS: usize = 80;

const COLS: usize = 25;
const MAX: usize = ROWS * COLS;

fn scroll() {
    unsafe {
        for i in 80..MAX {
            let src: *mut u8 = ((MMIO as usize) + (i * 2)) as *mut u8;
            let dst: *mut u8 = ((MMIO as usize) + ((i - 80) * 2)) as *mut u8;
            *dst = *src;
            *((dst as usize + 1) as *mut u8) = COLOR;
        }
        for i in (MAX-80)..MAX {
            let dst: *mut u8 = ((MMIO as usize) + ((i) * 2)) as *mut u8;
            *dst = 32;
            *((dst as usize + 1) as *mut u8) = COLOR;
        }
        LATEST = LATEST - 80;
    }
}

pub fn str_to_vga(s: &str) {
    let v = s.as_bytes();
    unsafe {
        for i in 0..v.len() {
            if LATEST > MAX {
                scroll();
            }
            match v[i] {
                10 => LATEST = ((LATEST / 80) + 1) * 80,
                _ => char_to_vga(v[i]),
            }
        }
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut d = Dummy { };
    d.write_fmt(args).unwrap();
}




