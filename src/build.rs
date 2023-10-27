extern crate cc;

fn main() {
    cc::Build::new().file("src/WinApi(C)/harddisk.c").compile("harddisk");
}