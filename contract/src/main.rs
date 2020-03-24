#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(allocator_api)]

mod buddy_allocation;
mod libc_alloc;

use ckb_contract_std::{setup};
const SMALL: usize = 32;
const LARGE: usize = 4096;
const COUNT: usize = 10;
const TIMES: usize = 20;

#[no_mangle]
pub fn main() -> i8 {
    // let mut allocator = buddy_allocation::new_alloc();
    let mut allocator = libc_alloc::new_alloc();

    for _ in 0..TIMES {
        let mut ptrs = [0 as *mut u8; COUNT];
        for i in 0..COUNT {
            ptrs[i] = allocator.malloc(SMALL);
        }
        for i in 0..COUNT {
            allocator.free(ptrs[i]);
        }
        for i in 0..COUNT {
            ptrs[i] = allocator.malloc(LARGE);
        }
        for i in 0..COUNT {
            allocator.free(ptrs[i]);
        }
    }
    0
}

setup!(main);
