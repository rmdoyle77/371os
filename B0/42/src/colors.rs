mod img;

pub fn colors() {
    for color in 0u8..16 {
        for row in 0..25 {
            for col in 0..5 {
                let pos = row * 80 + color as usize * 5 + col;
                unsafe {
                    let vga = 0xb8000 as *mut u16;
                    *vga.add(pos) = (color as u16) << 12 | (color as u16) << 8 | 0x20;
                }
            }
        }
    }
}

pub fn image() {
    unsafe {
        let vga = 0xb8000 as *mut u16;
        for (i, &color) in img::IMAGE.iter().enumerate() {
            let c = color as u16;

            *vga.add(i) = c << 12 | c << 8 | 0x20;
        }
    }
}

