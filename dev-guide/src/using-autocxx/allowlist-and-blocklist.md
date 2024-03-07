# Allowlist and Blocklist

The `include_cpp!` macro lists the desired classes and functions to be
translated to Rust, listed in a list of `generate!` and similar
derivatives.

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

Here is how we added `Router` class to the allowlist.

1. Find the header file which defines the `Router` class in C++ source
   code. You can use `grep -r 'class Router'` to search for it. Here
   we find `vanetza/geonet/router.hpp`,

2. Locate the namespace that the class belongs to. Here we find
   `vanetza::geonet::Router`.

  ```c++
  namespace vanetza
  {
  namespace geonet
  {
  class Router { /* .. */ };
  }
  }
  ```

3. Add the header path and `generate!` to the `include_cpp!` macro.

  ```rust
  include_cpp! {
      #include "vanetza/geonet/router.hpp"
      // ..
      generate!("vanetza::geonet::Router")
  }
  ```


You can see these derivatives, each serving different purposes.


## Derivatives

### `generate!`

The `generate!("vanetza::geonet::Router")` derivative adds the
`Router` class to the allowlist. It can also used to add C++ functions
and constants.

The generated Rust type is _opaque_. That is, the members of the
constructed object cannot be directly accessed in Rust. It can only be
pointed by a `UniquePtr` or a `Box` wrapper.

```rust
use autocxx::prelude::*;
use vanetza_sys::vanetza::geonet::Router;

let router: UniquePtr<Router> = Router::new(..).within_unique_ptr();
```

### `generate_pod!`

The `generate_pod!("vanetza::MacAddress")` derivative treats the added
class as a Plain Old Data type (POD). It will generate a plain struct
in Rust that can be directly constructed. It is recommended for simple
classes and enum classes.

For example, the `MacAddress` class (in header
`vanetza/vanetza/net/mac_address.hpp`) has only one array member.

```c++
class MacAddress : public boost::totally_ordered<MacAddress>
{
public:
    static constexpr std::size_t length_bytes = 6;
    // omit..
    std::array<uint8_t, length_bytes> octets;
};
```

In the Rust side, it can be constructed directly.

```rust
use vanetza_sys::vanetza::MacAddress;

let addr = MacAddress {
    _base = 0,
    octets: [1u8; 6]
};
```

### `generate_ns!`

The `generate_ns!("vanetza::aid")` derivative causes all classes,
functions and constants under the namespace to be generated. It is
useful for generating constants. For example,

For example, the constants listed in `vanetza/common/its_aid.hpp` can
be generated all at once using the derivative.

```c++
namespace vanetza
{
namespace aid
{

constexpr ItsAid CA = 36;
constexpr ItsAid DEN = 37;
constexpr ItsAid TLM = 137;
// ...

} // namespace aid
} // namespace vanetza
```

### `block!`

The `block!("vanetza::units::Acceleration")` derivative prevents a
class or a function to be generated. It is useful when a class or a
function causes compile error during code generation. This derivative
blacklists the offending class.
