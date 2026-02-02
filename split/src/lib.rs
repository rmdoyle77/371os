pub unsafe fn split_at_mut_unchecked<T>(
    values: &mut [T],
    mid: usize,
) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    assert!(mid <= values.len(), "mid out of bounds");
    // SAFETY: We just asserted the required precondition.
    unsafe { split_at_mut_unchecked(values, mid) }
}

