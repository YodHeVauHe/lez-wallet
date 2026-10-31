# Testing and sequencer checks

Run the workspace checks from the repository root:

```sh
cargo fmt --all -- --check
RISC0_DEV_MODE=1 cargo clippy --workspace --all-targets --locked -- -D warnings
RISC0_DEV_MODE=1 cargo test --workspace --locked
```

`lez-wallet status` performs only the sequencer's `getLastBlockId` RPC request. The
endpoint defaults to `https://testnet.lez.logos.co`; override it with
`--sequencer-url http://localhost:3040` for a local node. It sends no transactions.

## Local standalone sequencer

The upstream LEZ checkout documents standalone mode in its README and `Justfile`.
From a clone of `logos-execution-zone`, install the toolchain and native dependencies
listed by upstream. At the `v0.3.2` tag, run the documented standalone command (or its
`Justfile` equivalent): `RUST_LOG=info cargo run --features standalone -p sequencer_service -- lez/sequencer/service/configs/debug/sequencer_config.json`. The config points at the local Bedrock node on `http://localhost:18080`; standalone mode mocks the Bedrock clients. The RPC client default is `http://127.0.0.1:3040`. Keep the process running and query it from this repository with:

```sh
cargo run -p lez-wallet-cli -- status --sequencer-url http://127.0.0.1:3040
```

Upstream also documents a local Bedrock plus sequencer stack; its Docker compose build needs substantial disk and memory. In an environment without `libclang.so`, the wallet workspace cannot compile because upstream `librocksdb-sys` bindgen requires it. The standalone launch and CLI status check should be run after installing `libclang-dev`; this environment could not reach that step. The separate direct read-only testnet RPC probe is recorded in the PR verification section.
