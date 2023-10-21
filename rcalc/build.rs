fn main() {
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .compile("rcalc");

    // This line forces cxx to rerun you will get out
    // of sync other wise
    println!("cargo:rerun-if-changed=src/main.rs");
}
