# Contributing to LEZ Wallet

This guide is for everyone who works in this repository, human contributors and AI coding agents alike.

## How work is organised

- **Roadmap:** [`docs/roadmap.md`](docs/roadmap.md), derived from the [architecture and milestones page](https://cursor.com/canvas/store-4d755132-6c15-4451-a1d4-163afd036ef7/f1ff9f30-4a94-43c3-857c-bf730bc22112). Phase order is the build order.
- **Issues:** one issue per phase with goal, scope checklist, the LP-0021 criteria it covers, acceptance checks and dependencies. A tracking issue lists every phase. Adoption work has its own issues.
- **Labels:** `lez` (LEZ programs and chain integration) · `wallet` (engine and keystore) · `sdk` (Provider SDK) · `cli` · `backend` (core module) · `frontend` (QML UI, packaging) · `community` · `adoption` · `epic`.

## Ground rules

1. **Claim before you build.** Assign yourself or comment on the issue.
2. **Stay inside the issue.** Anything else needs its own discussion first.
3. **Never work on `main`.** Branch as `issue-<N>-<slug>`. PR titles follow `Phase N: <summary> (issue #N)` and the body says `Closes #N`, repeats the checklist, and lists verification evidence.
4. **Small PRs, one concern.** Docs-only changes stay separate from code where practical.
5. **Commits** follow the standard in [`AGENTS.md`](AGENTS.md): conventional subject with the issue number, then `Changes:` and `Verification:` lists.
6. **Tick checklist items** in the issue as they land; maintainers verify the acceptance checks at review.
7. **Checks:** `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` (with `RISC0_DEV_MODE=1` where proving is involved). See `docs/testing.md` once it exists.
8. **Privacy rules are design rules:** no telemetry, the sequencer is the only default network peer, optional services are disclosed and off by default.
9. **Ask in the issue or PR** when scope is unclear.

## Rules for AI coding agents

Same rules as humans, plus: read before acting; never force-push or push to `main`; never merge; do not create or close issues or labels; do not commit secrets; if blocked, stop and comment on the issue with the exact error rather than changing scope; never post to the Logos forum, Discord or X.
