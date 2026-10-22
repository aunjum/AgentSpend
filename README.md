# AgentSpend

**Your AI works hard. Your wallet shouldn't.**

AgentSpend is a local-first Rust CLI for importing AI coding-agent token usage from supported local session records. It scans available records and stores usage totals locally.

## Current status

This is an early MVP. Codex JSONL `token_usage_record` events and Claude Code assistant usage fields are imported when present. Costs, Gemma recommendations, and a live terminal dashboard are not implemented. The CLI does not claim savings or API charges from token counts.

## Install and start

Install Rust/Cargo, then run `cargo install --path .` (or `./install.sh`).

```sh
agentspend init
agentspend scan
agentspend status
agentspend report
```

`scan` walks local Codex and Claude session directories, imports explicit usage metadata, and stores it in SQLite under the platform local data directory. Prompt and response fields are ignored and not stored; credentials and agent configuration are not opened. `watch` repeats scans every ten seconds. See [CLI reference](docs/CLI_REFERENCE.md).

## Supported agents

| Agent | Detection | Usage import |
|---|---|---|
| OpenAI Codex CLI | Local session directory | Token usage records, when present |
| Claude Code | Local project directory | Assistant message usage and cache fields, when present |
| Gemini CLI | Not implemented | Not implemented |

## Privacy and costs

Collection is local and stores selected metadata in SQLite. Token counts do not establish actual spending. Cost, subscription allocation, and savings calculations are not implemented. Ollama/Gemma integration is also not implemented; no AI recommendations are fabricated.

Source fields are retained separately: Codex's reported input can include cached tokens, while Claude reports uncached input and cache counts separately. Do not add these fields across agents as if they shared a billing definition.

See [Privacy](docs/PRIVACY.md), [Limitations](docs/LIMITATIONS.md), and [Architecture](docs/ARCHITECTURE.md). A real demo should be recorded after running against local data; no screenshot is included because none was generated in this workspace.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

See [Testing](docs/TESTING.md) and [Contributing](CONTRIBUTING.md). Prepared for the Hacktoberfest 2026 Open-Source AI Challenge, Week 3: Slow and Steady. See [submission draft](docs/HACKATHON_SUBMISSION.md). MIT licensed.
