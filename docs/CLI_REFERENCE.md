# CLI reference

- `init`: create the local database.
- `scan`: import newly seen Codex usage events.
- `status`, `report`, `analyze`, `optimize`: scan and print aggregate token totals; these are aliases and do not generate cost or recommendations.
- `watch`: poll every ten seconds until interrupted.
- `dashboard`: print a one-shot overview, not a live TUI.
- `doctor`: show local directory detection and integration status.
- `--help`, `--version`: provided by clap.
