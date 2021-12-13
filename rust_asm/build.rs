use cc;

fn main() {
    cc::Build::new()
        .file("src/substart.s")
        .compile("asm");

    /*
    cc::Build::new()
        .file("src/print_b.s")
        .compile("asm");
    */
}
