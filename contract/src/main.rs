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

pub static mut HEAP_VECTOR:[u8;REQUIRED_SPACE] = [0u8;REQUIRED_SPACE];

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = unsafe{NonThreadsafeAlloc::new(HEAP_VECTOR.as_ptr())};

#[alloc_error_handler]
fn oom_handler(_: alloc::alloc::Layout) -> ! {
    exit(-128i8);
    loop{}
}

#[no_mangle]
pub extern "C" fn main() -> i8 {
    let v = vec![0u8;42];
    debug!("{:?}", v);
    0
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    exit(main());
    loop{}
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
