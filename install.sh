#!/usr/bin/env bash
set -euo pipefail
command -v cargo >/dev/null 2>&1 || { echo "Cargo is required. Install Rust from https://rustup.rs" >&2; exit 1; }
install_dir="$(cd "$(dirname "$0")" && pwd)"
bin_dir="${CARGO_HOME:-$HOME/.cargo}/bin"
if [[ -e "$bin_dir/agentspend" ]] && ! cargo install --list | grep -q '^agentspend v'; then
  echo "Refusing to overwrite an existing non-Cargo-managed file: $bin_dir/agentspend" >&2
  exit 1
fi
cargo install --path "$install_dir" --force
echo "Ensure ~/.cargo/bin is on PATH, then run: agentspend --help"
