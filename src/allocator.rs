use crate::{rust_aligned_alloc, rust_free};
use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("alloc memmory error {:?}", layout)
}

pub struct LibcAlloc;

unsafe impl GlobalAlloc for LibcAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        rust_aligned_alloc(layout.align(), layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        rust_free(ptr)
    }
}
