# Raw system calls for Rust

This library allows Rust code to invoke system calls directly.

See the [list of supported platforms](https://github.com/cptpcrd/scall/tree/master/src/platform).  Additions are very welcome!

We do not intend to provide wrapper functions like `read(2)` etc. because there are many subtly different ways to define them in Rust.
