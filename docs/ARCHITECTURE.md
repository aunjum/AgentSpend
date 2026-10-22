# Architecture

The CLI discovers expected local directories, recursively scans JSONL files, parses explicit Codex `token_usage_record` events and Claude assistant message usage records, and inserts selected metadata into SQLite with a source-derived deduplication key. The database is under the platform local data directory. `watch` polls every ten seconds. TUI, costing, and AI analysis remain future work.
