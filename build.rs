fn main() {
    println!("cargo:rustc-link-lib=dylib=vulnerable");
    println!("cargo:rustc-link-search=native=.");
}
