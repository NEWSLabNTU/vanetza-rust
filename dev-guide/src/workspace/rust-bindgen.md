# Binding Generation Process

Let's check the dependency list in `vanetza-sys/Cargo.toml`.

```bash
[dependencies]
autocxx = "0.26.0"
cxx = "1.0.118"

[build-dependencies]
autocxx-build = "0.26.0"
cmake = "0.1.50"
```

The `[build-dependencies]` section lists the dependencies for the
`build.rs` script. The `[dependencies]` section lists dependencies for
the output library. They are separated for purpose.

The [cmake](https://crates.io/crates/cmake) crate is used for library
compilation, provided that Vanetza is a CMake project. The
[autocxx-build](https://crates.io/crates/autocxx-build) is the binding
generator for C++. We'll employ this crate to scan through listed C++
classes, and it will produce analogous Rust data types. The generated
Rust code will use data types from
[autocxx](https://crates.io/crates/autocxx) and
[cxx](https://crates.io/crates/cxx). That's why we include them.


## The Build Script `build.rs` Explained

The script `vanetza-sys/build.rs` is executed before the compilation
of the `vanetza-sys` library. It is convenient for cases that we need
to compile non-Rust code. Our `build.rs` compiles the Vanetza source
code and it uses special instructions ask the compiler to link Vanetza
libraries. The details are explained in [Rust
book](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
and we'll focus on our implementation.

Let's check the `vanetza-sys/build.rs`. The main function runs two
major steps. It first compiles the Vanetza source code within the
`vanetza` submodule and returns the directory storing the compiled
Vanetza. Then, it scans header files within the directory and
generates Rust code.

```rust
fn main() {
    let vanetza_dir = build_vanetza();
    generate_rust_bindings(&vanetza_dir);
}
```

We'll move to each function and explain them in details.

## Compile Vanetza

Let's move to `build_vanetza()` function. 

```rust
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
```

The line `cmake::Config()...` calls CMake to build the Vanetza
project. It effectively the same with the following commands.

```bash
cmake ../vanetza
make -j
make install
```

The compiled Vanetza is installed to a directory and the path is
returned to `dst_dir`. The subdirectories under `dst_dir` follows the
UNIX convention. The `include` directory stores the header files, and
`lib` directory stores the compiled library files.

```
dst_dir/
├── include/
└── lib/
    ├── libvanetza_geonet.a
    ├── libvanetza_btp.a
    └── ...
```

The next step calls `println!()` to ask the Rust compiler to search for
Vanetza library files in the `$dst_dir/lib` directory. It is analogous
to `gcc -L $dst_dir/lib`.

```rust
// lib_dir := $dst_dir/lib
println!("cargo:rustc-link-search=native={}", lib_dir.display());
```

Then, link the libraries we desired. We have a list of libraries
stored in `VANETZA_LIBRARIES` global variable. It is analogous to `gcc
-l $lib`.

```rust
const VANETZA_LIBRARIES: &[&str] = &[
    "vanetza_access",
    "vanetza_asn1",
    // omit...
];

for lib in VANETZA_LIBRARIES {
    println!("cargo:rustc-link-lib=static={lib}");
}
```

The Rust compiler records the `println!()` output from build script
and performs actions. There are more special instructions and you can
see the [Rust
book](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script)
if you're interested.


## Rust FFI Code Generation

To allow Rust to access to compiled libraries, one has to write the
foreign function interface code in Rust. The code works like C/C++
headers. It only declares function signatures and data structures but
does not have function bodies.

For example, to access to `foo()` and `bar()` functions in
`libvanetza_abc.a`. The Rust FFI code writes

```rust
#[link(name = "vanetza_abc")]
extern {
   fn foo(a: u8) -> i32;
   fn bar();
}
```

In most cases, we don't write the FFI code manually. The C/C++ headers
are already in Vanetza source repository and we use automatic code
generator to translate the C++ headers into Rust FFI code.

Let's check the `generate_rust_bindings()` function to see how it
works.

```rust
fn generate_rust_bindings(vanetza_dir: &Path) {
    println!("cargo:rerun-if-changed=src/ffi_autocxx.rs");

    let include_dirs = { /* omitted */ };

    let mut cc_build = autocxx_build::Builder::new("src/ffi_autocxx.rs", include_dirs)
        .build()
        .expect("Unable to generate bindings");

    cc_build
        .flag_if_supported("-std=c++14")
        .compile("vanetza_rust");
}
```

The line `autocxx_build::Builder::new()` is the step that performs the
code generation. Before this step, it lists the directories to search
for headers and saves them in `include_dirs`. It reads the
`src/ffi_autocxx.rs` file and finds the `include_cpp!` macro. The
macro lists the C++ classes and functions for code generation.

```rust
include_cpp! {
    #include "vanetza/common/its_aid.hpp"
    #include "vanetza/common/position_fix.hpp"
    ...
    
    safety!(unsafe_ffi)
    
    generate!("vanetza::geonet::Router")
    generate!("vanetza::geonet::Area")
    ...
}
```

The autocxx code generator works for most simple C++ classes and
functions. However, it could fail on some complex types. We'll play
some tricks to work around this and we'll introduce them in the next
chapter.
