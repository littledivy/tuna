fn main() {
    println!("cargo:rustc-link-lib=static=soundio");
    println!("cargo:rustc-link-lib=asound");
    println!("cargo:rustc-link-lib=pulse");
    println!("cargo:rustc-link-lib=jack");
}
