#!/usr/bin/env sh
# Run the clippy linter on the code with all warnings enabled
cargo clean && cargo clippy -- -W clippy::all
