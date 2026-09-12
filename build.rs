fn main() {
    println!("cargo:rerun-if-changed=src/parser.c");
    println!("cargo:rerun-if-changed=src/tree_sitter/parser.h");
    cc::Build::new()
        .include("src")
        .file("src/parser.c")
        .flag_if_supported("-std=c11")
        .warnings(false)
        .compile("tree-sitter-v");
}
