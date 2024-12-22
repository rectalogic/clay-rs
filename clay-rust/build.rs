extern crate cc;

fn main() {
    cc::Build::new().file("clay.c").compile("clay");
}
