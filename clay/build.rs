extern crate cc;

fn main() {
    cc::Build::new()
        .file("clay.c")
        .std("c99")
        .extra_warnings(false)
        .compile("clay");
}
