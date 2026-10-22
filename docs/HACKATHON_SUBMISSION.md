---
title: "I Built AgentSpend: A Rust CLI That Automatically Tracks AI Agent Usage and Finds Wasted Tokens"
published: false
tags: devchallenge, hf26challenge
---

*This is a submission for the [Hacktoberfest Open-Source AI Challenge Week 3: Slow and Steady](https://dev.to/challenges/hacktoberfest-week3-2026-10-19).*

**Tags:** `devchallenge` `hf26challenge`

## What I Built

AgentSpend is a local-first Rust CLI that imports explicit Codex token usage records and Claude Code assistant usage fields from local session files, persisting deduplicated totals in SQLite. Manually hunting through agent records is tedious; the goal is to make collection automatic and private. This is an early MVP: costs, waste detection, and AI recommendations are not implemented.

## Demo

Install and reproduce with `cargo install --path .`, then `agentspend doctor`, `agentspend init`, `agentspend scan`, and `agentspend status` on a machine with supported session records. The CLI was built, installed locally, and run against existing Codex and Claude records; no usage totals are published because they are private machine data. No screenshot or recording is attached.

## Code

Repository: [github.com/aunjum/AgentSpend](https://github.com/aunjum/AgentSpend). Install with `cargo install --path .`.

## How I Built It

Rust provides a native CLI. Clap defines commands, serde parses relevant event metadata, and SQLite stores deduplicated usage locally. The connectors use turn-level Codex counters and Claude assistant message IDs where available to avoid repeat imports. Prompt and response fields are ignored and not stored; credentials are not opened. Parsing two evolving local formats while preserving separate cache counters and avoiding duplicate imports was the main implementation challenge. Gemma integration is planned, but not functional: no model is called and no AI recommendations are produced, so Gemma prize eligibility is not claimed.

## Why Does Open Innovation Matter?

Open-source local tools let developers inspect collection behavior, adapt connectors, and keep usage records on their machines.

## My Agent Session

Omitted: no measured agent-session development data is included.

## The Lazy Payoff

The scanner replaces repeated manual searches through supported Codex and Claude records with a local scan. Time saved has not been measured; no financial savings are claimed.

## Prize Categories

The overall challenge is the intended target, but this MVP does not yet meet the open-source AI/open-weight model theme. Gemma category eligibility is not claimed until a meaningful integration exists. This draft is not submission-ready until the missing AI feature and demo are completed.

## Limitations and testing

Gemini import, costs, a live TUI, and Ollama/Gemma analysis remain unimplemented. `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (4 tests), and `cargo build --release` passed. Existing local Codex and Claude records were imported; sanitized tests cover both schemas, malformed events, and deduplication. A separate Codex CLI task could not be run because the `codex` executable was not available on PATH. The current MVP does not yet meet the challenge's open-source AI/open-weight model theme, so this article is a draft rather than a ready submission.
