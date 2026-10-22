# Final release report

Status: locally installed early MVP. Release binary: `/home/mirinfo/.cargo/bin/agentspend`.

- Commands: init, scan, watch, status, report, analyze, optimize, doctor, dashboard (one-shot text), help, version.
- Connectors: Codex explicit JSONL token usage events and Claude assistant message usage fields. Gemini unsupported.
- Persistence: local SQLite; source line-derived idempotency key.
- Costs, savings, Gemma, live TUI: not implemented.
- Checks: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (4 passed), and `cargo build --release` passed after the Codex and Claude connector changes.
- CLI: cargo-installed, version/help and all finite commands were run successfully. `watch` was not run because it is an indefinite foreground process.
- Real collection: local Codex and Claude session history was imported; token totals appeared in status/report. Codex CLI executable was unavailable on PATH, so no isolated new Codex task was run.
- Duplicate prevention: repeated import of a sanitized JSONL fixture inserted one row total (automated test). Real source directories changed during active collection, so stable repeated-scan behavior was not measured against a quiescent corpus.
- Development data correction: a transient Codex key prefix produced duplicate rows during connector work. Removed 3,177 prefixed duplicates from the newly created local database while retaining canonical original rows; the collector now preserves the original key format.
- Security: no credentials intentionally accessed; collector makes no network requests. Database stores local usage metadata and source path-derived IDs. Secret-pattern search of project files returned no matches.
- Documentation: README, install, architecture, connector, privacy, limitations, demo, and submission drafts are present. No real screenshot/recording was created.
- Remaining: implement real cost and subscription separation, Gemini connector, live TUI, Ollama/Gemma, filesystem notifications, and create a privacy-safe real demo before full feature claims or submission.
