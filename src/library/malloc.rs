use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

unsafe extern "C" {
    unsafe static __heap_start: u8;
    unsafe static __heap_end: u8;
}

pub fn init_allocator() -> Result<(), &'static str> {
    unsafe {
        let heap_start = &raw const __heap_start as *mut u8;
        let heap_size = &raw const __heap_end as usize - heap_start as usize;
        ALLOCATOR.lock().init(heap_start, heap_size);
    }

    Ok(())
}
