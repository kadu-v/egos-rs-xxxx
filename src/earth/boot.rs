#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use egos_rs_xxxx::library::malloc::init_allocator;
use egos_rs_xxxx::{print, println};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("Panic occurred: {:?}", _panic);
    loop {}
}

#[unsafe(no_mangle)]
fn boot() -> ! {
    init_allocator().expect("Failed to initialize allocator");

    let b = Box::new(42);
    print!("Boxed value: {}\n", b);
    print!("Hello, world!");
    println!("This is a test of the println macro.");

    loop {}
}
