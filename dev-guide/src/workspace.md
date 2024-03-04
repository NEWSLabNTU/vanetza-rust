# Start a Workspace

## Prepare the Workspace

The workspace is stacked on three layers of code.

1. Vanetza C++ source code and headers.
2. Low-level Rust bindings to C++ classes and functions. (generated)
3. High-level Rust code wrapping around low-level Rust bindings.

The Vanetza C++ source code repository is already provided, we open a
Git submodule to pin to specific version. The low-level Rust bindings
and high-level wrapper will be separate libraries. Both are placed
within a Cargo workspace.

The workspace was prepared in these steps.

1. Create the project folder.

  ```bash
  mkdir project
  cd project
  ```

2. Pull the Vanetza submodule.

  ```bash
  git submodule add https://github.com/riebl/vanetza.git
  ```

3. Create the `Cargo.toml` in the project root to start a Cargo
   workspace.

  ```toml
  [workspace]
  resolver = "2"
  members = []
  ```
4. Create the `vanetza-sys` library for low-level Rust bindings.

  ```bash
  cargo new --lib vanetza-sys
  ```

4. Create the `vanetza` library, placed under `vanetza-rust` for
   high-level Rust wrapper.

  ```bash
  cargo new --lib --name vanetza vanetza-rust
  ```

The workspace layout should look like this.

```
project/
├── Cargo.toml
├── vanetza/
│   ├── **/*.hpp
│   └── **/*.cpp
├── vanetza-sys/
│   └── src/
└── vanetza-rust/
    └── src/
```

And the root `Cargo.toml` becomes

```toml
[workspace]
resolver = "2"
members = [
    "vanetza-sys",
    "vanetza-rust",
]
```

### The Workflow of Bindings Generation

To allow Rust to access the C++ classes and functions, the compiled
C++ library and the FFI code (foreign function interface) should be
prepared. On Linux platforms, the compiled library file can be either
a static library named `libname.a` or a shared library named
`libname.so`. The interface code is provided through headers.

The process is similar with the way we work with C/C++ projects, we
tell include paths and link paths to the Rust compiler, and ask the
compiler to link to specific libraries we want. You can see the
appendix section to learn how C/C++ links to external libraries.

The binding generation and library linking are handled by
`vanetza-sys` crate. We write the script `vanetza-sys/build.rs` to run
through these steps.

1. Build Vanetza library files. It produces `libvanetza_btp.a`,
   `libvanetza_geonet.a`, etc.
2. Scan Vanetza headers and generate Rust FFI code analogue of the
   headers.
3. Link to compiled Vanetza libraries in the output Rust library.

With binding generation, `vanetza-sys` turns to a library serving C++
functions and classes, but written in analogous Rust code. The Rust
code is unsafe to use. That's why we start a `vanetza-rust` wrapper
around the generated code.

### Rust Binding Generation

Let's edit the dependency list in `vanetza-sys/Cargo.toml`.

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

Open the `vanetza-sys/build.rs`. The main function runs two major
steps shown below. The other code is omitted for simplicity.

```rust
fn main() {
    let vanetza_dir = build_vanetza();
    generate_rust_bindings(&vanetza_dir);
}
```

TODO

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

TODO

```rust
fn generate_rust_bindings(vanetza_dir: &Path) {
    println!("cargo:rerun-if-changed=src/ffi.rs");

    let include_dirs = { /* omitted */ };

    let mut cc_build = autocxx_build::Builder::new("src/ffi.rs", include_dirs)
        .build()
        .expect("Unable to generate bindings");

    cc_build
        .flag_if_supported("-std=c++14")
        .compile("vanetza_rust");
}
```

### Appendix: Library Linking in C/C++

Let's see an example how C/C++ links to the external OpenCV library.

```bash
g++ \
    -I ~/my_opencv/include \
    -L ~/my_opencv/lib \
    -l opencv_core \
    main.cpp
```

Let's explain the compiler options.

The option  `-I` tells the directory to search for header files. For example, if
 we write the include code in C++, we have to make sure the
 file`~/my_opencv/include/opencv4/opencv2/core.hpp` exists.

```c++
#include <opencv4/opencv2/core.hpp>
```

The option `-L` tells the directory to search for library files, named
 `libname.a` and `libname.so` on Linux. It affects the later `-l`
 option.

The option `-l` asks the output binary to link to `opencv_core`. The
compiler will search for `libopencv_core.a` or `libopencv_core.so`
files in system default directories and those provided by `-L`. Here
we assume the file `~/my_opencv/lib/libopencv_core.a` exists.




