use cc;
fn main() {
    cc::Build::new()
        .file("csrc/test1.c")
        .include("csrc")
        .compile("test1");
}
