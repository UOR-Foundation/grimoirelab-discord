Add Discord backend

Perceval can now fetch the messages of a Discord channel or thread through the Discord HTTP API v10 with a bot token.

- Messages are fetched oldest first by the `after` snowflake, so a fetch resumes from a date.
- Pages hold at most 100 messages. A page that is too large, or that does not advance the cursor, is refused instead of looping.
- The client waits on `X-RateLimit-Remaining` and `X-RateLimit-Reset`, using the existing `RateLimitHandler`.
- The bot token is removed from archived requests, so fetches replay from an archive.
- `perceval discord <guild-id> <channel-id> -t <token> [--from-date]`

The bot only needs View Channel, Read Message History and the Message Content intent.

### Tests

`tests/test_discord.py` adds 28 tests, including archive replay. The whole suite (`tests/run_tests.py`, 1048 tests) and flake8 pass at d60867ab.

The changelog entry is `releases/unreleased/discord-backend-added.yml`, and the README lists the backend.

### Design artifacts

This contribution was modeled before it was written. The model, its proofs and the reviews it carries are in [UOR-Foundation/grimoirelab-discord](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/README.md):

- [Behavior](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/PercevalDiscord.lex.tex): the paging state machine (cursor, page limit, rate limit, refusals) and its [invariants, proved in Lean 4](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/PercevalDiscordProofs.lex.tex)
- [Security and threat model](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Security.lex.tex): STRIDE threats and their mitigations, including token handling in archives
- [Quality](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Quality.lex.tex): ISO/IEC 25010 requirements
- [Compliance](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Compliance.lex.tex): this project's contribution norms (DCO, changelog entry, tests, docs, lint, security advisories), each with its evidence
- [Architecture and stakeholder views](https://github.com/UOR-Foundation/grimoirelab-discord/blob/main/app/src/Architecture.lex.tex)
