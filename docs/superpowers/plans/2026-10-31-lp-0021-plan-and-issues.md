# Plan: repository setup and issues for LP-0021

1. Create `LOGOS-UGANDA/lez-wallet` with README, MIT and Apache-2.0 licenses, `.gitignore`, `AGENTS.md`, `CONTRIBUTING.md` and `docs/roadmap.md`. Verify: repository page renders, license detected.
2. Create labels (SACCO set plus `wallet`, `sdk`, `cli`, `adoption`, `epic`). Verify: `gh label list`.
3. Open the tracking issue, then one issue per phase (0–15) and three adoption issues, each with goal, scope checklist, quoted LP-0021 criteria, acceptance checks, dependencies and a link to the plan page. Verify: `gh issue list`.
4. Start Codex on the first engineering issue (Phase 1, setup and spikes) on branch `issue-<N>-<slug>`, with commits in the repository standard and a PR that closes the issue. Verify: PR exists, CI state recorded.
