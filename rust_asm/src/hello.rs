#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

extern "C" {
    fn print(pointer: *const u8);
}

fn prints(s: &str) {
    let p: *const u8 = s.as_ptr();
    for i in 0..(s.len()) {
        unsafe {
            print(p.offset(i as isize));
        }
    }
}

#[no_mangle]
fn hello() {
    let a = "h";
    unsafe {
        print(a.as_ptr());
    }
    prints("\n");
    prints("Hello World\n");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] 
#[no_mangle]
pub extern fn rust_eh_personality() {}

