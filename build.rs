extern crate gcc;

fn main() {
    gcc::Build::new().file("src/R/fexact.c").compile("fexact");
}
