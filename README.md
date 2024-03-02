# vatsim_utils

[![CI](https://github.com/Celeo/vatsim_utils/workflows/CI/badge.svg?branch=master)](https://github.com/celeo/vatsim_utils/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/vatsim_utils.svg)](https://crates.io/crates/vatsim_utils)
[![Docs.rs](https://docs.rs/vatsim_utils/badge.svg)](https://docs.rs/vatsim_utils)
[![License](https://img.shields.io/crates/l/vatsim_utils)](https://github.com/Celeo/vatsim_utils/blob/master/Cargo.toml#L10)

Utilities to interact with VATSIM data, covering:

- The live data from <https://data.vatsim.net/v3>
- API from <https://api.vatsim.net>
- Several thousand airport decimal latitude/longitude coordinations
- Various utility functions

This library is not endorsed by VATSIM or VATUSA.

## Installing

Add the latest version to your `Cargo.toml`.

## Using

[Docs link](https://docs.rs/vatsim_utils).

## Developing

### Building

### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/vatsim_utils
cd vatsim_utils
cargo test
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

Airport data from <https://www.partow.net/miscellaneous/airportdatabase/>.

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
