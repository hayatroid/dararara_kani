#!/bin/sh -e

[ "$CLAUDE_CODE_REMOTE" = true ] || exit 0

PATH=$(nix develop -c printenv PATH)
echo "export PATH=$PATH" >> "$CLAUDE_ENV_FILE"

# Must match kani-version in .github/workflows/ci.yaml.
cargo install --locked kani-verifier --version 0.68.0

cargo build

# Without this, each new session's first cargo kani sets up Kani and compiles everything.
cargo kani --only-codegen
