extern crate cc;

fn main() {
    cc::Build::new().file("src/C_Wrapper_Fn/harddisk.c").compile("harddisk");
}