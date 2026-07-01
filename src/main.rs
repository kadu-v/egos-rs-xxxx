#![no_main]
#![no_std]

use core::fmt;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

#[panic_handler]
#[inline(never)]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

fn terminal_write(s: &str) {
    let ptr = 0x10000000 as *mut u8;
    for byte in s.bytes() {
        unsafe {
            write_volatile(ptr, byte);
        }
    }
}

fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    struct Writer;

    impl Write for Writer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            terminal_write(s);
            Ok(())
        }
    }

    let mut writer = Writer;
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    print!("Hello, world!");

    println!("This is a test of the println macro.");

    loop {}
}
