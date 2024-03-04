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

## License

This project is licensed under LGPLv3, see [license file](LICENSE.md)
for details.
