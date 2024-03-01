use std::env;
use std::path::{Path, PathBuf};

const VANETZA_LIBRARIES: &[&str] = &[
    "vanetza_access",
    "vanetza_asn1",
    "vanetza_asn1_its",
    "vanetza_asn1_pki",
    "vanetza_asn1_security",
    "vanetza_asn1_support",
    "vanetza_btp",
    "vanetza_common",
    "vanetza_dcc",
    "vanetza_facilities",
    "vanetza_geonet",
    "vanetza_gnss",
    "vanetza_net",
    "vanetza_security",
];

fn main() {
    let vanetza_dir = build_vanetza();
    build_c_wrapper(&vanetza_dir);
    generate_rust_bindings();
}

fn build_vanetza() -> PathBuf {
    let dst_dir = cmake::Config::new("../vanetza").build_arg("-j").build();
    let lib_dir = dst_dir.join("lib");

    // Link Vanetza libraries
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    for lib in VANETZA_LIBRARIES {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    dst_dir
}

fn build_c_wrapper(vanetza_dir: &Path) {
    cc::Build::new()
        .cpp(true)
        .include(vanetza_dir.join("include"))
        .file("csrc/vanetza_c.cpp")
        .compile("vanetza_c");
}

fn generate_rust_bindings() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("bindings.rs");

    let bindings = bindgen::Builder::default()
        .header("csrc/vanetza_c.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings.");
}
