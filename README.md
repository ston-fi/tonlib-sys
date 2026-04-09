# Rust Bindings for tonlibjson Library

This repository contains the Rust bindings for tonlib library (https://github.com/ton-blockchain/ton), allowing developers to use tonlib functionality in their Rust applications.
Starting from version 2024.6.1, this project depends on a forked version of ton instead of the original  (https://github.com/ston-fi/ton). This change was made to address urgent fixes and potential future needs.

## Features
* Uses Cmake to build tonlibjson_static by default.
* Supports shared tonlib. You can build with --features shared-tonlib.

## Usage
This library is used in the tonlib-rs library (https://github.com/ston-fi/tonlib-rs), which provides a higher-level Rust interface to the tonlib functionality.

To use this library in your Rust application, add the following to your Cargo.toml file:

```toml
[dependencies]
tonlib-sys = "2026.2.1"
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
