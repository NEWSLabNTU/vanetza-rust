# Rust Wrapper for Vanetza

This project provides the Rust wrapper of the
[vanetza](https://github.com/riebl/vanetza) library, an open-source
ETSI C-ITS protocol suite. It comprises of these directories:

- `vanetza`: The C++ Vanetza repository.
- `vanetza-sys`: The Rust FFI interface for Vanetza library.
- `vanetza-rust`: The high-level Rust bindings for Vanetza library.
- `dev-guide`: The developer guide.


## Usage

Modify `Cargo.toml` to use this library in your Rust project.

```bash
[dependencies.vanetza]
git = "https://github.com/NEWSLabNTU/vanetza-rust.git"
```

To open the API doc of this library, download this repository and run.

```bash
cargo doc --open
```

## For Developers

Please read the book in [dev-guide](dev-guide).

## Troubleshooting

Currently, the library only works with Clang lower than version 14. If
you're using newer operating system such as Ubuntu 22.04, it's
recommended to configure the compiler to clang-12.

```rust
sudo apt install clang-12 libclang-12-dev
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-12
export LIBCLANG_PATH=/usr/lib/llvm-12/lib
export LIBCLANG_STATIC_PATH=/usr/lib/llvm-12/lib
export CLANG_PATH=/usr/bin/clang-12
```

## License

This project is licensed under LGPLv3, see [license file](LICENSE.md)
for details.
