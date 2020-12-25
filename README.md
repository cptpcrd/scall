# scall

[![crates.io](https://img.shields.io/crates/v/scall.svg)](https://crates.io/crates/scall)
[![Docs](https://docs.rs/scall/badge.svg)](https://docs.rs/scall)
[![GitHub Actions](https://github.com/cptpcrd/scall/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/cptpcrd/scall/actions?query=workflow%3ACI+branch%3Amaster+event%3Apush)
[![Cirrus CI](https://api.cirrus-ci.com/github/cptpcrd/scall.svg?branch=master)](https://cirrus-ci.com/github/cptpcrd/scall)
[![codecov](https://codecov.io/gh/cptpcrd/scall/branch/master/graph/badge.svg)](https://codecov.io/gh/cptpcrd/scall)

Friendlier raw system calls for Rust.

This is a fork of [`sc`](https://crates.io/crates/sc) with a slightly reworked API. Advantages:

1. It's easier to use -- the `syscall!` macro returns a `Result<usize, i32>` indicating either the syscall result (on success) or error number (on failure).

2. It properly supports x86_64 FreeBSD and macOS. `sc` *technically* supports x86_64 FreeBSD and macOS; however, those OSes have different conventions for returning error values, which `sc` fails to acknowledge. This effectively renders `sc` useless for most purposes on those OSes.

*See the [list of supported platforms](https://github.com/cptpcrd/scall/tree/master/src/platform).*
