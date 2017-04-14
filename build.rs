extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Let the gcc crate handle all the C library compilation and linking
    gcc::compile_library("liblibrary.a", &["library/library.c"]);

    // Use the bindgen builder to create a binding, adding options
    let bindings = bindgen::Builder::default()
        .raw_line("#[allow(improper_ctypes)]") // what does this do?
        .generate_comments(true)
        // Output bindings for builtin definitions, e.g. __builtin_va_list (which mpc uses)
        .emit_builtins()
        // Emit no unstable/nightly Rust code
        .no_unstable_rust()
        // The input header we would like to generate bindings for
        .header("library/library.h")
        // Finish the builder and generate the bindings
        .generate()
        // Unwrap the Result and panic on failure
        .expect("Unable to generate bindings!");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
