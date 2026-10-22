#!/usr/bin/env bash
set -euo pipefail
bin="${CARGO_HOME:-$HOME/.cargo}/bin/agentspend"
if [[ ! -e "$bin" ]]; then echo "AgentSpend executable not found at $bin"; exit 0; fi
command -v cargo >/dev/null 2>&1 || { echo "Cannot confirm Cargo ownership; refusing to remove $bin" >&2; exit 1; }
cargo install --list | grep -q '^agentspend v' || { echo "Refusing to remove a file not recorded by cargo install: $bin" >&2; exit 1; }
read -r -p "Remove $bin? Usage database will be kept. [y/N] " answer
[[ "$answer" == y || "$answer" == Y ]] || { echo "Cancelled."; exit 0; }
rm -- "$bin"
echo "Removed executable. Local usage database was kept."
