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

// In src/interrupts.rs

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    let t = TICKS.fetch_add(1, Ordering::Relaxed);
    
    unsafe {

        if t % crate::snake::TICK_RATE == 0 {
            crate::snake::update();
        }
    }
    
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let scancode: u8 = unsafe { x86_64::instructions::port::Port::new(0x60).read() };
    if let Ok(Some(key_event)) = unsafe { (*core::ptr::addr_of_mut!(KB)).add_byte(scancode) } {
        if let Some(key) = unsafe { (*core::ptr::addr_of_mut!(KB)).process_keyevent(key_event) } {
            match key {
                pc_keyboard::DecodedKey::Unicode(c) => {
                    match c {
    'w' | 'W' => unsafe {
        if crate::snake::DIRECTION != b'S' { crate::snake::DIRECTION = b'W'; }
    },
    'a' | 'A' => unsafe {
        if crate::snake::DIRECTION != b'D' { crate::snake::DIRECTION = b'A'; }
    },
    's' | 'S' => unsafe {
        if crate::snake::DIRECTION != b'W' { crate::snake::DIRECTION = b'S'; }
    },
     'd' | 'D' => unsafe {
        if crate::snake::DIRECTION != b'A' { crate::snake::DIRECTION = b'D'; }
                     },
                      'q' | 'Q' => crate::snake::quit(),
                    _ => {}
                    }
                    unsafe { crate::snake::update(); }
                }
                _ => {}
            }
        }
    }
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8) };
}
