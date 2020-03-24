pub struct CAllocator;

extern {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

impl CAllocator {
    pub fn malloc(&mut self, size: usize) -> *mut u8 {
        unsafe{malloc(size)}
    }
    pub fn free(&mut self, ptr: *mut u8) {
        unsafe{free(ptr)}
    }
}

pub fn new_alloc() -> CAllocator {
    CAllocator{}
}
