#!/usr/bin/env bash
# Builds the transpiled crate (release) with the pinned nightly toolchain in
# rust-toolchain.toml. Requires only rustup + cargo — no c2rust/Docker needed
# (unlike transpile.sh, this step works on any architecture).
#
# Invoke as: ./rust-migration/build-crate.sh
set -euo pipefail
cd "$(dirname "$0")/.."

CRATE=rust-migration/transpiled
if [ ! -d "${CRATE}" ]; then
	echo "ERROR: ${CRATE} not found; run rust-migration/transpile.sh first." >&2
	exit 1
fi

echo "==> Building the crate (release)"
( cd "${CRATE}" && rm -f Cargo.lock && cargo build --release )

echo "==> Running cargo test"
( cd "${CRATE}" && cargo test --release )
