# Allowlist and Blocklist

TODO

```rust
// The macro lists C++ headers to be processed and allowed C++ items.
// This block is replaced to a `mod ffi {...}` in compile time.
include_cpp! {
    // -------- Includes --------
    #include "vanetza/common/its_aid.hpp"
    #include "vanetza/geonet/router.hpp"
    #include "vanetza_wrapper.hpp"

    // -------- Safet Declaration --------
    safety!(unsafe_ffi)

    // -------- Allowlist --------
    generate!("vanetza::geonet::Router")
    generate_pod!("vanetza::ItsAid")
    generate_ns!("vanetza_wrapper")

    // -------- Forbidden Items --------
    block!("vanetza::units::Acceleration")
}
```

TODO

### `generate!("vanetza::geonet::Router")`

TODO

### `generate_pod!("vanetza::ItsAid")`

TODO

### `generate_ns!("vanetza_wrapper")`

TODO

### `block!("vanetza::units::Acceleration")`

TODO
