use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=SDL2");

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <SDL2/SDL.h>")
        .blocklist_item("FP_.+")
        .derive_debug(false)
        .generate_comments(false)
        .layout_tests(false)
        .merge_extern_blocks(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .generate()
        .expect("failed to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("sdl_bindings.rs");

    bindings.write_to_file(out).expect("failed to write bindings");
}
