use crate::print;
use crate::println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub static PICS: spin::Mutex<pic8259::ChainedPics> =
    spin::Mutex::new(unsafe { pic8259::ChainedPics::new(32, 40) });

pub fn init_idt() {
    unsafe {
        (*core::ptr::addr_of_mut!(IDT)).breakpoint.set_handler_fn(breakpoint_handler);
        (&mut (*core::ptr::addr_of_mut!(IDT)))[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        (&mut (*core::ptr::addr_of_mut!(IDT)))[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_handler);
        (*core::ptr::addr_of_mut!(IDT)).load();
static mut KB: pc_keyboard::Keyboard<pc_keyboard::layouts::Us104Key, pc_keyboard::ScancodeSet1> =
    pc_keyboard::Keyboard::new(
        pc_keyboard::ScancodeSet1::new(),
        pc_keyboard::layouts::Us104Key,
        pc_keyboard::HandleControl::Ignore,
    );

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let scancode: u8 = unsafe { x86_64::instructions::port::Port::new(0x60).read() };
    if let Ok(Some(key_event)) = unsafe { (*core::ptr::addr_of_mut!(KB)).add_byte(scancode) } {
        if let Some(key) = unsafe { (*core::ptr::addr_of_mut!(KB)).process_keyevent(key_event) } {
            match key {
                pc_keyboard::DecodedKey::Unicode(c) => {
                    match c {
                        '\u{0008}' => print!("\x08 \x08"), // backspace
                        _ => print!("{}", c),
                    }
                }
                pc_keyboard::DecodedKey::RawKey(_) => {}
            }
        }
    }
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8) };
}    }
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}



