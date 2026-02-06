use core::mem::{size_of, MaybeUninit};
use core::ptr;

pub const SIZE: usize = 1024;

static mut BUS: [u8; SIZE] = [0u8; SIZE];

const WORD: usize = 8;

#[inline]
const fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

#[inline]
const fn words() -> usize {
    SIZE / WORD
}

#[inline]
const fn bitmap_bytes() -> usize {
    div_ceil(words(), 8)
}

#[inline]
const fn bitmap_words() -> usize {
    div_ceil(bitmap_bytes(), WORD)
}

// raw read from BUS without creating references
unsafe fn bus_read(i: usize) -> u8 {
    let base = core::ptr::addr_of!(BUS) as *const u8;
    unsafe { *base.add(i) }
}

// raw write to BUS without creating references
unsafe fn bus_write(i: usize, v: u8) {
    let base = core::ptr::addr_of_mut!(BUS) as *mut u8;
    unsafe { *base.add(i) = v; }
}

unsafe fn bit_get(word_i: usize) -> bool {
    let byte = word_i >> 3;
    let bit = (word_i & 7) as u8;
    (bus_read(byte) & (1u8 << bit)) != 0
}

unsafe fn bit_set(word_i: usize, used: bool) {
    let byte = word_i >> 3;
    let bit = (word_i & 7) as u8;
    let mask = 1u8 << bit;

    let cur = bus_read(byte);
    let next = if used { cur | mask } else { cur & !mask };
    bus_write(byte, next);
}

unsafe fn init() {
    if bit_get(0) {
        return;
    }

    for i in 0..bitmap_bytes() {
        bus_write(i, 0);
    }

    for w in 0..bitmap_words() {
        bit_set(w, true);
    }
}

pub fn malloc(s: usize) -> Option<usize> {
    if s == 0 {
        return None;
    }

    unsafe {
        init();

        let need = div_ceil(s, WORD);
        let start = bitmap_words();
        let total = words();

        if need > total.saturating_sub(start) {
            return None;
        }

        for w in start..=total - need {
            let mut ok = true;
            for k in 0..need {
                if bit_get(w + k) {
                    ok = false;
                    break;
                }
            }

            if ok {
                for k in 0..need {
                    bit_set(w + k, true);
                }
                return Some(w * WORD);
            }
        }

        None
    }
}

pub fn setter<T>(val: T, loc: usize) {
    unsafe {
        let n = size_of::<T>();

        let src = (&val as *const T) as *const u8;
        let dst_base = core::ptr::addr_of_mut!(BUS) as *mut u8;
        let dst = dst_base.add(loc);

        ptr::copy_nonoverlapping(src, dst, n);
    }
}

pub fn getter<T>(loc: usize) -> T {
    unsafe {
        let n = size_of::<T>();

        let mut out = MaybeUninit::<T>::uninit();
        let dst = out.as_mut_ptr() as *mut u8;

        let src_base = core::ptr::addr_of!(BUS) as *const u8;
        let src = src_base.add(loc);

        ptr::copy_nonoverlapping(src, dst, n);
        out.assume_init()
    }
}

