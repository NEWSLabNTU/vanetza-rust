# Start a Workspace

## Prepare the Workspace

The workspace is stacked on three layers of code.

1. Vanetza C++ source code and headers.
2. Low-level Rust bindings to C++ classes and functions. (generated)
3. High-level Rust code wrapping around low-level Rust bindings.

The Vanetza C++ source code repository is already provided. We open a
Git submodule to pin to specific version. The low-level Rust bindings
and high-level wrapper will be separate libraries. Both are placed
within a Cargo workspace.

The workspace was prepared in these steps. You don't have to go
through these steps because they're already done but it's for your
reference.

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

## The Workflow of Bindings Generation

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

