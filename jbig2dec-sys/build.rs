use std::fs;

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        println!("Try `git submodule update --init --recursive`");
        panic!();
    }
}

fn main() {
    fail_on_empty_directory("jbig2dec");

    println!("cargo:rerun-if-changed=config.h");
    println!("cargo:rerun-if-changed=config_types.h");
    let src_files = vec![
        "jbig2dec/jbig2_arith.c",
        "jbig2dec/jbig2_arith_int.c",
        "jbig2dec/jbig2_arith_iaid.c",
        "jbig2dec/jbig2_huffman.c",
        "jbig2dec/jbig2_hufftab.c",
        "jbig2dec/jbig2_segment.c",
        "jbig2dec/jbig2_page.c",
        "jbig2dec/jbig2_symbol_dict.c",
        "jbig2dec/jbig2_text.c",
        "jbig2dec/jbig2_halftone.c",
        "jbig2dec/jbig2_generic.c",
        "jbig2dec/jbig2_refinement.c",
        "jbig2dec/jbig2_mmr.c",
        "jbig2dec/jbig2_image.c",
        "jbig2dec/jbig2.c",
    ];
    for file in &src_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    cc::Build::new()
        .include(".")
        .include("jbig2dec/")
        .files(src_files)
        .warnings(false)
        .extra_warnings(false)
        .compile("libjbig2dec.a");
    println!("cargo:rustc-link-lib=static=jbig2dec");
}
