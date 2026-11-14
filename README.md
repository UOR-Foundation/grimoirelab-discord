# GrimoireLab Discord: the contribution, modeled as a Prism

This repository models one upstream contribution to
[GrimoireLab](https://chaoss.github.io/grimoirelab/) 2.0, in CHAOSS branding:

- a [Perceval](https://github.com/chaoss/grimoirelab-perceval) backend that
  reads a Discord guild channel through the Discord API v10, and
- a [Chronicler](https://github.com/chaoss/grimoirelab-chronicler) eventizer
  that turns those items into CloudEvents.

The model states the backend's paging behavior as a state machine, proves its
invariants in Lean 4, and carries the security, threat, quality and compliance
models that the upstream's maintainers, users and security reviewers each need.

## Single source

`app/prompt.json` is the only hand-written model file. Everything under
`app/src/`, `site/brand/upstream/`, `features/` and `model/` is its build, by
the builder that `prism/builder.lock.json` pins:

```
prism/build.sh          # regenerate from app/prompt.json
prism/build.sh --check  # refuse a generated file that is stale or hand-edited
just vv                 # verify everything in the pinned SDK
```

The `prism-build` workflow runs `--check` on every change to a model path.

## Artifacts

| Stakeholder concern | Artifact |
| --- | --- |
| Behavior: paging, cursor, rate limits, refusals | `app/src/PercevalDiscord.lex.tex`, proofs in `app/src/PercevalDiscordProofs.lex.tex` |
| Security (STRIDE threats, one accepted with reason) | `app/src/Security.lex.tex` |
| Quality (ISO/IEC 25010) | `app/src/Quality.lex.tex` |
| Compliance (DCO, changelog, tests, docs, lint, advisories) | `app/src/Compliance.lex.tex` |
| Architecture and stakeholder views | `app/src/Architecture.lex.tex`, `app/src/Design.lex.tex` |
| Upstream branding | `site/brand/upstream/` |

## The code

The upstream code is verified against GrimoireLab's own test suites at pinned
commits in [UOR-Foundation/community](https://github.com/UOR-Foundation/community/tree/claude/project-thread-3qbcxu/oracles/grimoirelab):

- `perceval-discord/`: the backend, 28 tests with archive replay, and a changelog entry
- `chronicler-discord/`: the eventizer, 27 tests, and its docs section
- `upstream.patch`: the README and help-text lines Perceval needs

## License

MIT OR Apache-2.0. The vendored CHAOSS marks keep their own licenses.

## Upstream pull requests

`upstream/perceval-pr.md` and `upstream/chronicler-pr.md` are the drafted
bodies of the two upstream pull requests. Each body links the artifacts above.
