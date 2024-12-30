use cc;
fn main() {
    cc::Build::new()
        .file("csrc/test1.c")
        .include("csrc")
        .shared_flag(true)
        .compile("test1.so");
}