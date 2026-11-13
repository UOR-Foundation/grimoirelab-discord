Add Discord eventizer

Chronicler can now turn the items of the Perceval `discord` backend into CloudEvents:

- `org.grimoirelab.events.discord.message`
- `.message.edited`
- `.message.reply`
- `.thread.created`
- `.message.reaction`

Authors and mentioned users are given as identity events (`.message.authored_by`, `.message.mentioned`). These events keep Discord's `bot` flag, so bot activity can be told apart from people's.

### Tests

`tests/test_discord.py` adds 27 tests. The suite and ruff pass at 522221d2.

The changelog entry is `releases/unreleased/discord-eventizer-added.yml`, and the events documentation gains a Discord section.

Depends on the Perceval Discord backend (chaoss/grimoirelab-perceval#TBD).

### Design artifacts

This contribution was modeled before it was written. The model, its proofs and the reviews it carries are in [UOR-Foundation/grimoirelab-discord](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/README.md):

- [Behavior](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/PercevalDiscord.lex.tex): the paging state machine (cursor, page limit, rate limit, refusals) and its [invariants, proved in Lean 4](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/PercevalDiscordProofs.lex.tex)
- [Security and threat model](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Security.lex.tex): STRIDE threats and their mitigations, including token handling in archives
- [Quality](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Quality.lex.tex): ISO/IEC 25010 requirements
- [Compliance](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Compliance.lex.tex): this project's contribution norms (DCO, changelog entry, tests, docs, lint, security advisories), each with its evidence
- [Architecture and stakeholder views](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Architecture.lex.tex)
