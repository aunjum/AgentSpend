# Connectors

## Codex

Reads `~/.codex/sessions/**/*.jsonl`, accepts `token_usage_record` events, and prefers `turn_token_usage` to avoid importing cumulative thread counters. Events lacking usage are ignored. IDs derive from source path and line number, so repeated scans are idempotent. Schema changes may cause records to be skipped.

Codex's reported input count can include its cached input count; these fields overlap and must not be added together.

## Claude Code

Reads `~/.claude/projects/**/*.jsonl`, importing assistant messages only when explicit usage metadata is present. Input, output, cache read, and cache creation counts are persisted separately. Assistant message UUID is used for deduplication when available. The local JSONL format may change; these counts do not prove API billing.

Claude reports uncached input separately from cache counts, so its input field is not directly comparable to Codex's reported input field.

## Gemini CLI

Not implemented. No configuration or credentials are read.
