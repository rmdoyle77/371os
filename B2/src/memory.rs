use x86_64::structures::paging::PageTable;

pub unsafe fn active_level_4_table(offset: x86_64::VirtAddr)
    -> &'static mut x86_64::structures::paging::PageTable
{
    unsafe {
        let (frame, _) = x86_64::registers::control::Cr3::read();
        let phys = frame.start_address();
        let virt = offset + phys.as_u64();
        let ptr: *mut PageTable = virt.as_mut_ptr();
        return &mut *ptr;
    }
}

pub unsafe fn init(
    offset: x86_64::VirtAddr,
) -> x86_64::structures::paging::OffsetPageTable<'static> {
    unsafe {
        let l4 = active_level_4_table(offset);
        x86_64::structures::paging::OffsetPageTable::new(l4, offset)
    }
}
