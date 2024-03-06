use std::env;
use std::path::PathBuf;

fn main() {
    let eclipse_dir = PathBuf::from(env::var("ECLIPSEDIR").expect("ECLIPSEDIR enviroment varaibel not found"));
    let mut dll_folder = eclipse_dir.clone();
    dll_folder.push("lib/x86_64_linux");
    let mut include_path = eclipse_dir.clone();
    include_path.push("include/x86_64_linux");
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}",dll_folder.to_str().unwrap());
    println!("cargo:rustc-link-lib=eclipse");
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.  
        .header("wrapper.h")
        .allowlist_recursively(false)
        .allowlist_file(".*eclipse.h")
        .allowlist_file(".*ec_public.h")
        .allowlist_file(".*wrapper.h")
        .allowlist_file(".*types.h")
        .allowlist_file(".*embed.h")
        .allowlist_file(".*ec_general.h")
        .allowlist_file(".*os_support.h")
        .allowlist_file(".*config.h")
        .detect_include_paths(false)
        .opaque_type("pthread_.*")
        .opaque_type("__pthread.*")
        .opaque_type("__pthread_cond_s")
        .no_debug("shared_data_t")
        .blocklist_file(".+stddef\\.h")
        .blocklist_file(".*sched\\.h")
        .blocklist_file(".*math\\.h")
        .blocklist_file(".*float\\.h")
        .blocklist_file(".*pthread\\.h")
        .blocklist_file(".*assert\\.h")
        .blocklist_file(".*stdint\\.h")
        //.depfile("deps.txt")
        .clang_arg(format!("-I{}",include_path.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
