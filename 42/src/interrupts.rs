use crate::vga::_print;
use crate::println;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        (*core::ptr::addr_of_mut!(IDT)).breakpoint.set_handler_fn(breakpoint_handler);
        (*core::ptr::addr_of_mut!(IDT)).load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

