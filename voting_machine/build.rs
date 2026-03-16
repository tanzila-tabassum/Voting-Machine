fn main() {
    println!("cargo:rustc-link-search=native=libs"); // Replace "libs" with the relative path to your sqlite3.lib
    println!("cargo:rustc-link-lib=static=sqlite3");
}
