# Agent instructions

Read `CONTRIBUTING.md`, `docs/roadmap.md` and your assigned issue before changing anything.

## GitHub commit writing standard

Every commit, by humans or agents, uses this exact structure with real newlines:

```text
Concise conventional subject (include the related issue number when known)

Changes:
- Describe concrete changes.

Verification:
- List commands and results, or explain why verification was not run.
```

Include the body. Never use literal `\n` text, and never add trailers. This also applies to commits prepared by delegated agents.

## Hard rules

- Work on `issue-<N>-<slug>`; never push to `main`, never force-push, never rewrite pushed history, never merge your own PR.
- Stay inside the issue's checklist. No repo-wide reformatting or drive-by refactors.
- Do not create, close or mass-edit issues or labels; propose them in a comment.
- Record real command output as verification. If something cannot be run, say so and why.
- Never commit seeds, keys, wallet storage or tokens.
- Do not post on the Logos forum, Discord or X on anyone's behalf.
