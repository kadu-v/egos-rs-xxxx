#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use egos_rs_xxxx::malloc::init_allocator;
use egos_rs_xxxx::{print, println};

#[panic_handler]
#[inline(never)]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("Panic occurred: {:?}", _panic);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    init_allocator().expect("Failed to initialize allocator");

    let b = Box::new(42);
    print!("Boxed value: {}\n", b);
    print!("Hello, world!");
    println!("This is a test of the println macro.");

    loop {}
}
