use std::{
    env, fs,
    path::{Path, PathBuf},
};

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
    "vanetza_btp_write",
];

fn main() {
    #[cfg(feature = "docs-only")]
    return;

    // Build the Vanetza C++ library and get the directory where
    // Vanetza header and library files are installed.
    let vanetza_dir = build_vanetza();

    // Generate Rust FFI code by translating C headers in `csrc`.
    generate_rust_bindings(&vanetza_dir);

    // Link to GeographicLib, a required dependency of Vanetza.
    link_geographic_lib();
    link_crypto_lib();
    link_lib();
    save_bindings();
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

fn generate_rust_bindings(vanetza_dir: &Path) {
    println!("cargo:rerun-if-changed=src/ffi_autocxx.rs");

    let include_dirs = {
        let vanetza_include_dir = vanetza_dir.join("include");
        let wrapper_include_dir = PathBuf::from("csrc");

        // HACK: The additional include path avoids "asn_application.h"
        // header not found error.
        let asn1_support_include_dir = vanetza_include_dir
            .join("vanetza")
            .join("asn1")
            .join("support");

        [
            vanetza_include_dir,
            asn1_support_include_dir,
            wrapper_include_dir,
        ]
    };

    let mut cc_build = autocxx_build::Builder::new("src/ffi_autocxx.rs", include_dirs)
        .build()
        .expect("Unable to generate bindings");

    cc_build
        .flag_if_supported("-std=c++14")
        .compile("vanetza_rust");
}

fn link_geographic_lib() {
    let lib = pkg_config::probe_library("geographiclib")
        .expect("Unable to find GeographicLib. Is it installed on your system?");

    for link_path in lib.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.display());
    }

    for lib in lib.libs {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}
fn link_crypto_lib() {
    let lib = pkg_config::probe_library("libcrypto++")
        .expect("Unable to find libcrypto. Is it installed on your system?");

    for link_path in lib.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.display());
    }
    for lib in lib.libs {
        println!("cargo:rustc-link-lib=static={lib}");
    }
}

fn link_lib() {
    // let link_lib = ["dbus-1","systemd","liblzma","libzstd","liblz4","libpcap","libgps"];
    // for link in link_lib.iter() {
    //     let lib = pkg_config::probe_library(*link)
    //         .expect("Unable to find lib. Is it installed on your system?");

    //     for link_path in lib.link_paths {
    //         println!("cargo:rustc-link-search=native={}", link_path.display());
    //     }

    //     for lib in lib.libs {
    //         println!("cargo:rustc-link-lib=static={lib}");
    //     }
    // }
    println!("cargo:rustc-link-search=native=/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=static=boost_program_options");
}

fn save_bindings() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let src_file = out_dir.join("autocxx-build-dir/rs/autocxx-ffi-default-gen.rs");
    let tgt_dir = manifest_dir.join("generated");
    let tgt_file = tgt_dir.join("bindings.rs");

    fs::create_dir_all(&tgt_dir)
        .unwrap_or_else(|_| panic!("Unable to create directory {}", tgt_dir.display()));
    fs::copy(src_file, &tgt_file)
        .unwrap_or_else(|_| panic!("Unable to create file {}", tgt_file.display()));
}
