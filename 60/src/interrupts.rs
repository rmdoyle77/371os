use crate::print;
use crate::println;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
static mut KB: pc_keyboard::Keyboard<pc_keyboard::layouts::Us104Key, pc_keyboard::ScancodeSet1> =
    pc_keyboard::Keyboard::new(
        pc_keyboard::ScancodeSet1::new(),
        pc_keyboard::layouts::Us104Key,
        pc_keyboard::HandleControl::Ignore,
    );

pub static PICS: spin::Mutex<pic8259::ChainedPics> =
    spin::Mutex::new(unsafe { pic8259::ChainedPics::new(32, 40) });

pub static TICKS: AtomicUsize = AtomicUsize::new(0);
pub static DIGIT_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static CLOCK_RUNNING: AtomicBool = AtomicBool::new(false);
pub static START_TICKS: AtomicUsize = AtomicUsize::new(0);
pub static INPUT_DIGITS: spin::Mutex<[u8; 6]> = spin::Mutex::new([0u8; 6]);

pub fn init_idt() {
    unsafe {
        (*core::ptr::addr_of_mut!(IDT)).breakpoint.set_handler_fn(breakpoint_handler);
        (&mut (*core::ptr::addr_of_mut!(IDT)))[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        (&mut (*core::ptr::addr_of_mut!(IDT)))[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_handler);
        (*core::ptr::addr_of_mut!(IDT)).load();
    }
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let scancode: u8 = unsafe { x86_64::instructions::port::Port::new(0x60).read() };
    if let Ok(Some(key_event)) = unsafe { (*core::ptr::addr_of_mut!(KB)).add_byte(scancode) } {
        if let Some(key) = unsafe { (*core::ptr::addr_of_mut!(KB)).process_keyevent(key_event) } {
            match key {
                pc_keyboard::DecodedKey::Unicode(c) => {
                    if !CLOCK_RUNNING.load(Ordering::Relaxed) {
                        match c {
                            '0'..='9' => {
                                let count = DIGIT_COUNT.load(Ordering::Relaxed);
                                if count < 6 {
                                    let mut digits = INPUT_DIGITS.lock();
                                    digits[count] = c as u8 - b'0';
                                    drop(digits);
                                    DIGIT_COUNT.fetch_add(1, Ordering::Relaxed);
                                    print!("{}", c);
                                    if count + 1 == 6 {
                                        START_TICKS.store(TICKS.load(Ordering::Relaxed), Ordering::Relaxed);
                                        CLOCK_RUNNING.store(true, Ordering::Relaxed);
                                    }
                                }
                            }
                            ' ' => { print!(" "); }
                            _ => {}
                        }
                    }
                }
                pc_keyboard::DecodedKey::RawKey(_) => {}
            }
        }
    }
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8) };
}
