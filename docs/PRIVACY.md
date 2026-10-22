# Privacy

Collection is local. AgentSpend parses selected Codex/Claude event fields, timestamps, session identifiers, model labels, and token usage. JSONL source lines are read, while prompt and response fields are ignored and not stored. Credentials and agent configuration are not opened. SQLite contains token counts and source-derived identifiers.
