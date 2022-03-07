use std::arch::{asm, global_asm};

global_asm!(
    ".global put_char",
    "put_char:",
    "mov rdx, rsi", // arg2 -> arg3: size of char
    "mov rsi, rdi", // arg1 -> arg2: pointer of buffer
    "mov rdi, 1",   // file discriptor
    "mov rax, 1",   // write
    "syscall",
    "ret",
);

extern "C" {
    fn put_char(p: *const u8, size: usize);
}

fn main() {
    println!("Hello, world!");
    let msg = "Hello World from global_asm!\n";
    unsafe {
        put_char(msg.as_ptr(), msg.len());
    }
    let msg = "Hello World from asm!\n";
    unsafe {
        asm!(
            "mov rdi, 1", // file discriptor
            "mov rsi, {0}", // pointer of buffer
            "mov rdx, {1}", // size of char
            "mov eax, 1", // write
            "syscall",
            in(reg) msg.as_ptr(),
            in(reg) msg.len(),
        );
    }
    print("Hello from wrapped func\n");

    let x: u64 = 0xff;
    let res: usize;
    unsafe {asm! {
        "popcnt {1}, {0}",
        in(reg) x,
        out(reg) res,
    }}
    print(format!("{}\n", res).as_str());
}

fn print(msg: &str) {
    unsafe {
        asm!(
            "mov rdi, 1", // file discriptor
            "mov rsi, {0}", // pointer of buffer
            "mov rdx, {1}", // size of char
            "mov eax, 1", // write
            "syscall",
            in(reg) msg.as_ptr(),
            in(reg) msg.len(),
        );
    }
}
