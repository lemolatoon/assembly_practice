#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

extern "C" {
    fn print(pointer: *const u8);
    fn print_char(c: u8);
}

fn write_str(s: &str) {
    let p: *const u8 = s.as_ptr();
    for i in 0..(s.len()) {
        unsafe {
            print(p.offset(i as isize));
        }
    }
}

fn write_str2(s: &str) {
    for c in s.chars() {
        unsafe {
            print_char(c as u8);
        }
    }
}

#[no_mangle]
fn hello() {
    let a = 'h';
    unsafe {
        print("h".as_ptr());
    }
    write_str("\n");
    write_str("Hello World\n");
    unsafe {
        print_char('h' as u8);
    }
    write_str2("Hello From `print_char!!!\n");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] 
#[no_mangle]
pub extern fn rust_eh_personality() {}

