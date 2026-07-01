#![no_main]
#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;
use egos_rs_xxxx::{print, println};

#[panic_handler]
#[inline(never)]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.enter")]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!("la sp, __heap_end", "call main");
    }
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    print!("Hello, world!");
    println!("This is a test of the println macro.");

    loop {}
}
