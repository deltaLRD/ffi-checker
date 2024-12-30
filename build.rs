use cc;
fn main() {
    cc::Build::new()
        .file("csrc/test1.c")
        .compile("test1.so");
}