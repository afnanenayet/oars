#!/usr/bin/env sh
# Run the clippy linter on the code with all warnings enabled
# that don't emit false positives
cargo clean && cargo clippy -- -W clippy::all
