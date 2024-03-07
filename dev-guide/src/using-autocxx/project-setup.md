# Project Setup

Come back to the `vanetza-sys` crate.

```
vanetza-sys/
├── Cargo.toml
├── build.rs                  (1)
├── csrc/vanetza_wrapper.hpp  (2)
└── src/
    ├── ffi_autocxx.rs        (3)
    └── lib.rs                (4)
```

1. The build script that performs code generation.
2. Extra C++ patches and wrappers. We'll talk about it later.
3. The file with `include_cpp!` allowlist. The `build.rs` reads this
   file and generate FFI code accordingly.
4. The entry of the Rust library.

## The `include_cpp!` Macro

The `include_cpp!` macro in `ffi_autocxx.rs` lists allowed and blocked
C++ classes and functions (items). It is processed by `build.rs`. The
macro block is replaced with generated Rust bindings.

```rust
// Import prelude required by generated code.
use autocxx::prelude::*;

// The macro lists C++ headers to be processed and allowed C++ items.
// This block is replaced to a `mod ffi {...}` in compile time.
include_cpp! {
    // -------- Includes --------
    #include "vanetza/common/its_aid.hpp"

    // -------- Safet Declaration --------
    safety!(unsafe_ffi)

    // -------- Allowlist --------
    generate_ns!("vanetza::aid")

    // -------- Forbidden Items --------
    block!("vanetza::units::Acceleration")
}

// Re-export things within the `ffi` module.
pub use ffi::*;
```

The `lib.rs` re-exports all generated items to the root module.

```rust
mod ffi_autocxx;
pub use ffi_autocxx::*;
```

## Review Generation Results

It's recommended to view the Rust document to see the outcome of code
generation.

```bash
cd vanetza-sys
cargo doc --open
```

