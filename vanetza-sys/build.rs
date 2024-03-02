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
    // Build the Vanetza C++ library and get the directory where
    // Vanetza header and library files are installed.
    let vanetza_dir = build_vanetza();

    // Build the C wrapper within `csrc` directory, requiring a
    // compiled Vanetza.
    build_c_wrapper(&vanetza_dir);

    // Generate Rust FFI code by translating C headers in `csrc`.
    generate_rust_bindings();
}

fn build_vanetza() -> PathBuf {
    // Compile the Vanetza source code using CMake.
    let dst_dir = cmake::Config::new("../vanetza").build_arg("-j").build();

    // Link Vanetza libraries
    let lib_dir = dst_dir.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    for lib in VANETZA_LIBRARIES {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    dst_dir
}

fn build_c_wrapper(vanetza_dir: &Path) {
    // Link GeographicLib, required by Vanetza.
    link_geographic_lib();

    // Trigger re-run if the C wrapper code is changed.
    println!("cargo:rerun-if-changed=csrc/vanetza_c.cpp");
    let include_dir = vanetza_dir.join("include");

    // HACK: The additional include path avoids "asn_application.h"
    // header not found error.
    let asn1_support_include_dir = include_dir.join("vanetza").join("asn1").join("support");

    // Build and link the C wrapper source code.
    cc::Build::new()
        .cpp(true)
        .includes([include_dir, asn1_support_include_dir])
        .file("csrc/vanetza_c.cpp")
        .compile("vanetza_c");
}

fn generate_rust_bindings() {
    // Trigger re-run if C wrapper header is changed.
    println!("cargo:rerun-if-changed=csrc/vanetza_c.hpp");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("bindings.rs");

    // Generate Rust bindings.
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

fn link_geographic_lib() {
    // Link to GeographicLib, a required dependency of Vanetza.
    let lib = pkg_config::probe_library("geographiclib")
        .expect("Unable to find GeographicLib. Is it installed on your system?");

    for link_path in lib.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.display());
    }

    for lib in lib.libs {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}
