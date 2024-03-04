# C++ to Rust Mapping

The translation from C++ namespace to Rust module is straightforward.
Let's look at the `Router` class and see how it's done in practice.

```bash
cd $repo/vanetza
vim vanetza/geonet/router.hpp
```

This is the simplified content of this header file.

```c++
namespace vanetza
{
    namespace geonet
    {
        /* (1) class */
        class Router
        {
            /* (2) constructor */
            Router(Runtime&, const MIB&);

            /* (3) descructor */
            ~Router();

            /* (4) method */
            DataConfirm request(const ShbDataRequest&, DownPacketPtr);

            /* (5) overloaded method */
            DataConfirm request(const GbcDataRequest&, DownPacketPtr);

            /* (6) inner class */
            enum class PacketDropReason { /* ... */ };
        };

        /* (7) function */
        std::string stringify(Router::PacketDropReason pdr);
    }
}
```

The table shows how these C++ items are translated to Rust. More
details can be found in
[Naming](https://google.github.io/autocxx/naming.html) section in the
autocxx book.

| No | C++ Item                                            | Rust Item                                       |
|----|-----------------------------------------------------|-------------------------------------------------|
| 1  | `Router` class                                      | `struct vanetza::geonet::Router` struct         |
| 2  | Constructor of `Router`                             | `Router::new(..)` method                        |
| 3  | Destructor of `Router`                              | `Router::drop()` destructor                     |
| 4  | `request()` method                                  | `Router::request(..)` method                    |
| 5  | `request()` overloading method                      | `Router::request1(..)` method                   |
| 6  | `PacketDropReason` inner enum class within `Router` | `vanetza::geonet::Router_PacketDropReason` enum |
| 7  | `stringify()` function                              | `vanetza::geonet::stringify()` function         |


