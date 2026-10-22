# Testing

Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `cargo build --release`. Automated tests parse Codex and Claude usage fields, malformed records, and fixture deduplication. Real session contents must not be committed.
