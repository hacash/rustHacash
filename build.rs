extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/x16rs/x16rs.c")
        .warnings(false)
        .flag("-Wunused-but-set-variable")
        .compile("libx16rs.so");
}