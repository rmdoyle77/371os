static mut LATEST: usize = 0;
const MMIO: usize = 0xb8000;
const COLOR: u8 = 0x0F;

pub fn char_to_vga(a: u8) {
    unsafe {
        let rel: *mut u8 = ((MMIO as usize) + (LATEST * 2)) as *mut u8;
        *rel = a;
        *((rel as usize + 1) as *mut u8) = COLOR;
        LATEST = LATEST + 1;
    }
}

pub fn str_to_vga(s: &str) {
    let v = s.as_bytes();
    for i in 0..v.len() {
        char_to_vga(v[i]);
    }
}





