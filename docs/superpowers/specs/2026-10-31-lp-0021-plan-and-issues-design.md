# Design record: LP-0021 plan and issue structure

## Objective

Turn the LP-0021 architecture and milestone plan into a repository whose work is tracked as one GitHub issue per phase, each delivered through its own pull request, following the pattern used in LOGOS-UGANDA/transparent-sacco.

## Decisions

- **Monorepo, dual license.** One repository holds the engine, CLI, Basecamp modules, SDK, mini-apps and testimonial program, under MIT or Apache-2.0 (a submission requirement).
- **Build on upstream.** The engine wraps the official LEZ `wallet` crate pinned to a release tag rather than reimplementing keys, storage or proving.
- **Issues mirror the weekly milestones.** Phase 0 is scope and validation (forum questions, testnet 0.3 vs 0.4, toolchain); Phases 1–15 follow the plan's weeks W0–W14+. Adoption work, which runs across phases, has three separate issues. A tracking issue lists all of them.
- **Labels:** SACCO's `lez`, `backend`, `frontend`, `community`, `review`, plus `wallet`, `sdk`, `cli`, `adoption`, `epic`.
- **Branches and PRs:** `issue-<N>-<slug>`; PR title `Phase N: <summary> (issue #N)`; body has `Closes #N`, the checklist and verification evidence. Never merged by the authoring agent.
- **Dates in issues.** Unlike SACCO, issue titles carry the week, because the adoption clock makes calendar timing part of the requirement.

## Out of scope for this record

Implementation details for each phase; those go in per-issue specs and plans under `docs/superpowers/`.
