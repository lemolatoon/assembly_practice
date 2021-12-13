fn main() {
    cc::build::new()
        .file("src/substart.s")
        .file("src/print_b.s")
        .compile();
}
