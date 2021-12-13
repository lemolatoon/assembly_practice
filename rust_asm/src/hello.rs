#![no_std]
#![no_main]
#![feature(lang_items)]

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

use core::panic::PanicInfo;

extern "C" {
    fn print(pointer: *const u8);
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Writer.write_fmt(args).unwrap();
}

struct Writer;
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let p: *const u8 = s.as_ptr();
        for i in 0..(s.len()) {
            unsafe {
                print(p.offset(i as isize));
            }
        }
        Ok(())
    }
}

#[no_mangle]
fn hello() {
    let a = "h";
    unsafe {
        print(a.as_ptr());
    }
    // use core::fmt::Write;
    print!("\n");
    print!("Hello World\n");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[lang = "eh_personality"] 
#[no_mangle]
pub extern fn rust_eh_personality() {}

