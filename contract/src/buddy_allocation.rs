use buddy_alloc::buddy_alloc::BuddyAlloc;

const _HEAP_SIZE: usize = 64 * 1024;
static mut _HEAP: [u8; _HEAP_SIZE] = [0u8; _HEAP_SIZE];

pub fn new_alloc() -> BuddyAlloc {
    unsafe{
        BuddyAlloc::new(_HEAP.as_ptr(), _HEAP_SIZE, 16)
    }
}
