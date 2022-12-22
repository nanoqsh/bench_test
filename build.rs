fn main() {
    println!("cargo:rerun-if-changed=clib.c");

    cc::Build::new()
        .compiler("clang")
        .file("clib.c")
        .flag("-O3")
        .compile("clib");
}
