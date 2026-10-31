# LEZ Wallet and Provider SDK

A wallet for the Logos Execution Zone (LEZ) that runs as a Logos Basecamp app and as a CLI, plus a Provider SDK that lets other Basecamp apps connect to it, pick an account, and ask the user to approve transactions they can read. Built for the Logos λPrize [LP-0021: LEZ Wallet and Provider SDK](https://github.com/logos-co/lambda-prize/blob/master/prizes/LP-0021.md).

> **Status:** planning and scaffolding. Nothing here is usable yet. Work is tracked as one GitHub issue per phase; see the [tracking issue](../../issues) and [`docs/roadmap.md`](docs/roadmap.md).

## Scope

- **Accounts:** many public and private LEZ accounts on one seed, with labels and an account switcher.
- **Assets:** own, send and receive native LEZ and fungible token-program assets from public and private accounts.
- **Provider SDK:** a documented way for third-party Basecamp apps to request account access, read balances and propose transfers or program calls. Every request shows a selection prompt and an approval prompt with a plain-language summary of what will move, to whom, and whether the program's source is verified.
- **Reference mini-apps:** a testimonial app (writes testimonials to an on-chain LEZ program through the full connect, select, approve flow) and a faucet app (funds an account chosen through the SDK and handles rate limits).
- **CLI:** every wallet action scriptable from the terminal; drives the end-to-end tests and the demo.
- **Target network:** the canonical Logos LEZ zone on testnet 0.4 (development starts on 0.3, which is what the live testnet runs today).

## Architecture

A Rust **engine** crate wraps the official LEZ `wallet` crate, pinned to a release tag, so key storage, account derivation, proving and sync come from upstream rather than being reimplemented. The engine owns the keystore (Argon2id + AES-GCM at rest), the account manager, a transaction builder with an IDL-based call decoder, a program-source verifier, and per-dApp session grants. Its only default network peer is the sequencer the user configures.

Two front ends share the engine. The **CLI** is a Rust binary. The **Basecamp wallet** is a Rust core module built with `logos-module-builder` plus a `ui_qml` app; the UI is the Basecamp app-to-app intent provider for the `lezwallet.*` namespace and renders the selection and approval prompts itself. Third-party apps, including the testimonial and faucet mini-apps, talk to the wallet only through those intents. Modules are published to a module catalog forked from `logos-modules-release-base`.

Optional services (faucet, Logos Storage encrypted backup, an LP-0023-style program registry) are disclosed, off by default, and the wallet stays fully functional without them. There is no telemetry.

The full plan, with the criteria checklist, milestones and risks, is in the [architecture and milestones page](https://cursor.com/canvas/store-4d755132-6c15-4451-a1d4-163afd036ef7/f1ff9f30-4a94-43c3-857c-bf730bc22112) and summarised in [`docs/roadmap.md`](docs/roadmap.md).

## Repository layout (planned)

| Path | Contents |
|---|---|
| `crates/engine` | Wallet engine on the upstream LEZ `wallet` crate |
| `crates/cli` | `lez-wallet` command-line interface |
| `modules/lez_wallet_core` | Basecamp core module (Rust) |
| `modules/lez_wallet_ui` | Basecamp wallet app and intent provider (QML) |
| `sdk/` | Provider SDK: intent schema, QML/JS helper, template module |
| `apps/testimonial`, `apps/faucet` | Reference mini-apps |
| `programs/testimonial` | LEZ testimonial program |
| `docs/` | Roadmap, design records, user and SDK guides |

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`AGENTS.md`](AGENTS.md) first. Work happens on branches named `issue-<N>-<slug>` and reaches `main` only through reviewed pull requests.

## License

Dual-licensed under [MIT](LICENSE) or [Apache-2.0](LICENSE-APACHE), at your option, as LP-0021 requires.
