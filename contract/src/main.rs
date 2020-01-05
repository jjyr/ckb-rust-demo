#![no_std]
#![no_main]
#![feature(start)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(const_raw_ptr_to_usize_cast)]

use buddy_alloc::{NonThreadsafeAlloc, REQUIRED_SPACE};
use ckb_contract_std::{syscalls::{exit, }, debug};
use alloc::vec;

extern crate alloc;

#[link_section = ".heap.mem"]
#[no_mangle]
pub static mut HEAP_VECTOR:[u8;REQUIRED_SPACE] = [0u8;REQUIRED_SPACE];

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = unsafe{NonThreadsafeAlloc::new(HEAP_VECTOR.as_ptr())};

#[alloc_error_handler]
fn oom_handler(_: alloc::alloc::Layout) -> ! {
    exit(-128i8);
    loop{}
}

#[no_mangle]
#[start]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let v = vec![0u8;42];
    debug!("{:?}", v);
    exit(0);
    // just ignore this return value
    0
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    debug!("{:?}", panic_info);
    exit(-100i8);
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}
