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

## Scaffold verification observations

In the implementation environment, the built CLI successfully queried the live testnet:
`nix develop -c ./target/debug/lez-wallet status` printed `Last block ID: 71654`.
A local sequencer was already listening on `127.0.0.1:3040`; the same CLI with
`--sequencer-url http://127.0.0.1:3040` printed `Last block ID: 1031`. That process was
launched from a separate SACCO scaffold, so this confirms the read-only client against a
local RPC endpoint but does not confirm this checkout's own standalone launch.

We attempted the upstream v0.3.2 standalone launch using the documented command and a
config whose state directory was under this repository's `target/`. Its Cargo lockfile
required a second dependency build. We stopped at 3.8 GB free on `/workspace` to stay
within the disk limit; the standalone process from this checkout did not start.
