# Installation

Install Rust with Cargo, then run `cargo install --path .` or `./install.sh`. The executable is normally placed in `~/.cargo/bin`, which must be on `PATH`.

`./uninstall.sh` removes only the installed executable after confirmation; it does not remove usage data. SQLite is stored in the platform local data directory (Linux typically `~/.local/share/agentspend/usage.sqlite3`).
