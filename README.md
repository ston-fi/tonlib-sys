# Rust Bindings for tonlibjson Library

This repository contains the Rust bindings for tonlib library (https://github.com/ton-blockchain/ton), allowing developers to use tonlib functionality in their Rust applications.
The default source build in version 2026.11.0 uses the upstream [TON v2026.11 release](https://github.com/ton-blockchain/ton/releases/tag/v2026.11).

## TON 2026.11 compatibility

Crate versions track the upstream TON year and month. This update removes the upstream
payment-channel JSON API, including `pchan.signPromise`, `pchan.validatePromise`,
`pchan.packPromise`, `pchan.unpackPromise`, and `actionPchan`. Applications using these
requests must remain on an older TON release or migrate before upgrading. The Rust
FFI function signatures are unchanged. Use `~2026.10.0` to stay on the previous release;
a requirement such as `"2026"` permits this update.

With `shared-tonlib`, the linked system library determines the TON version.

## Features
* Uses Cmake to build tonlibjson_static by default.
* Supports shared tonlib. You can build with --features shared-tonlib.

## Usage
This library is used in the tonlib-rs library (https://github.com/ston-fi/tonlib-rs), which provides a higher-level Rust interface to the tonlib functionality.

To use this library in your Rust application, add the following to your Cargo.toml file:

```toml
[dependencies]
tonlib-sys = "2026"
```

Then, in your Rust code, you can import the library with:

```rust
use tonlib_sys;
```

## Build

To speed up the build process, the TON monorepo is cloned only once into the ./cargo/git/db/ directory using an fs-lock.

If the cloned repository becomes inconsistent and causes build issues, you can manually remove the TON folder from ./cargo/git/db/ and retry the build.


## Contributing

If you want to contribute to this library, please feel free to open a pull request on GitHub.

## License
This library is licensed under the MIT license. See the LICENSE file for details.
