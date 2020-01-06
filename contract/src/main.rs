#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

#[no_mangle]
pub fn _start() -> ! {
    exit(0)
}

/// Exit syscall
/// https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md
pub fn exit(_code: i8) -> ! {
    unsafe {
        // a0 is _code
        asm!("li a7, 93");
        asm!("ecall");
    }
    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub fn abort() -> ! {
    panic!("abort!")
}
