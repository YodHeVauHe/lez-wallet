# Roadmap: LEZ Wallet and Provider SDK (LP-0021)

Derived from the [architecture and milestones page](https://cursor.com/canvas/store-4d755132-6c15-4451-a1d4-163afd036ef7/f1ff9f30-4a94-43c3-857c-bf730bc22112). Spec: [LP-0021](https://github.com/logos-co/lambda-prize/blob/master/prizes/LP-0021.md). Weeks start Monday, Africa/Kampala time; dates may move, phase order is the build order.

## Why this order

The adoption clock is the long pole: no submission qualifies until two months of on-chain testimonials exist. A basic wallet, the testimonial app and the faucet ship in Phase 4 (week of Nov 23) so December and January can be the counted months. Readable approvals, verification and backup land later as updates to an app users already have.

## Phases

| Phase | Week of | Theme | Done when |
|---|---|---|---|
| 0 | Oct 31 | Scope and validate | `docs/decisions.md` lists every question with an answer or a stated assumption; LEZ pin and 0.4 cutover approach are written down; Toolchain confirmed on Linux and the macOS plan is known |
| 1 | Nov 2 | Setup and spikes | `cargo build --workspace` and `cargo test --workspace` pass with the upstream wallet crate resolved from the pinned tag; CI is green on the PR; `lez-wallet status` prints the sequencer's last block ID from a local standalone sequencer (and from the live testnet RPC, read-only); Hello-world core and UI modules load in Basecamp; intent round trip demonstrated (screenshot or log) |
| 2 | Nov 9 | Engine and testimonial program | CLI sends a public transfer on the local sequencer and on testnet (tx hashes recorded); CLI writes a testimonial on testnet; program ID recorded in `docs/deployments.md`; Program and engine tests pass in CI |
| 3 | Nov 16 | Usable wallet v0 and Provider v0 | Hallway test: a non-expert installs and sends in under 5 minutes; A test dApp's transfer is refused without approval and succeeds with it; `.lgx` assets build from a clean clone |
| 4 | Nov 23 | Public alpha: wallet, testimonial and faucet | Modules install from the catalog through the package-manager UI; A fresh user funds an account via the faucet app and writes a testimonial via the testimonial app on testnet; E2E job green in CI |
| 5 | Nov 30 | Tokens and private flows | Token transfer both ways (public and private) on testnet, tx hashes recorded; Benchmark numbers in `docs/performance.md` |
| 6 | Dec 7 | SDK v1 and developer kit | An outside developer builds a demo app from the template in an afternoon (notes recorded); SDK docs walk through both mini-apps |
| 7 | Dec 14 | Readable approvals | Every lez-programs instruction renders as plain text in the approval prompt (fixture tests); Unlimited approval is flagged in a test |
| 8 | Dec 21 | Program verification | Prompt shows the correct status for a verified, a pinned and an unknown program (tests) |
| 9 | Dec 28 | Reliability and backup | Kill -9 and offline tests pass in CI; Backup and restore round trip demonstrated against Logos Storage |
| 10 | Jan 4 (date set by Logos) | Testnet 0.4 cutover | Full flow green on testnet 0.4; Gas shown in prompt and history (or documented as unavailable on 0.4) |
| 11 | Jan 11 | Hardening | CI green on the default branch with the full E2E suite |
| 12 | Jan 18 | Builder sprint | 10+ third-party apps in progress, each in its own public repo with genuine history |
| 13 | Jan 25 | Docs and demo | A fresh VM runs the demo script unmodified; Video recorded and linked |
| 14 | Feb 1 | Evidence and self-assessment | Every criterion evidenced except the testimonial clock |
| 15 | Feb 8 onward | Submit | Submission opened |

## Adoption track

- **150 on-chain testimonials over two months**: Reach 150+ (target 200) on-chain testimonials from distinct, genuinely active accounts, with 30+ in each of at least two months (target: December and January).
- **10 independent third-party Basecamp apps on the SDK**: Get 10 independent developers to ship Basecamp ui modules that use the Provider SDK, each with genuine commit history.
- **Discord and X testimonials and macOS coverage**: Collect 30+ genuine Discord and 30+ X testimonials (target 40 each) and show real macOS use.

## Design rules

- Reuse the upstream LEZ `wallet` crate for keys, storage, proving and sync; no custom crypto.
- The configured sequencer is the only default network peer. No telemetry. Faucet, Logos Storage backup and registry lookups are optional, disclosed and off by default.
- Nothing is signed or submitted without a selection and approval record; private balances are never shared without a per-account grant.
- Dual-licensed MIT or Apache-2.0.

## Risks

- A competitor qualifies first (first-come-first-served).
- Testnet 0.4 date, or a chain reset that drops testimonials.
- Ten independent developers is the hardest criterion.
- Solo capacity across Rust, Nix, Qt and RISC Zero.
- No accepted LP-0023 registry yet.
- Private proving speed, faucet rate limits, unsigned intents, Sybil scrutiny, macOS builds.
