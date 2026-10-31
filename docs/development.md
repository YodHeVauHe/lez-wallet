# Development setup

## Toolchain

The workspace pins Rust 1.98.1 in `rust-toolchain.toml`, matching the upstream LEZ
`v0.3.2` tag. That is the newest published `v0.3.x` tag found for the LEZ repository;
the `v0.3.2` wallet sources use edition 2024 and pin the same toolchain. The README
and LEZ wallet defaults identify `https://testnet.lez.logos.co` as the current testnet
endpoint. We pin `v0.3.2` to stay on the live testnet-compatible 0.3 line rather than
the `v0.4.0-rc1` prerelease intended for the later testnet cutover.

The engine depends on the upstream `wallet` package and its `sequencer_service_rpc`
package at the same tag. The latter builds the upstream RPC transport; the engine
uses its generated `RpcClient` trait to query the last block ID. The wallet package
does not expose its wallet-specific transport trait in v0.3.2.
This scaffold does not initialize wallet storage or manage keys.

## Build

Install Rust 1.98.1 with `rustup toolchain install 1.98.1 --component clippy --component rustfmt`.
On Nix, enter the development shell with `nix develop` (it sets `RUSTUP_TOOLCHAIN`).
Then run:

```sh
cargo build --workspace --locked
cargo run -p lez-wallet-cli -- --version
cargo run -p lez-wallet-cli -- config show
```

On Debian/Ubuntu the native packages used by upstream dependencies are `build-essential`,
`pkg-config`, `libssl-dev`, `clang`, `libclang-dev`, `cmake`, `protobuf-compiler`, and
`libpcsclite-dev`. Keep `CARGO_TARGET_DIR`
inside this checkout (the default `target/` is ignored by git). RISC Zero guest toolchains
are not needed to build the wallet package itself; if future workspace packages build guest
programs, install `rzup` and follow their specific setup instructions.
