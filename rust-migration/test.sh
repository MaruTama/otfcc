#!/usr/bin/env bash
# Convenience wrapper: build-crate.sh + run-cycles.sh. Runs the project's own
# round-trip stability tests against the Rust binaries, mirroring quick.make's
# ttfroundtriptest/cffroundtriptest targets.
#
# Invoke as:
#   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
#       --entrypoint bash otfcc-c2rust rust-migration/test.sh
#
# Requires rust-migration/transpiled/ to already exist (run transpile.sh
# first). Requires `node` on the host for tests/ttf-roundtrip-test.js —
# that step runs OUTSIDE this container, so this script only builds the
# crate and produces the dump/build cycle artifacts under build/; run the
# `node tests/ttf-roundtrip-test.js` comparisons on the host afterward (see
# the loop this script prints at the end).
set -euo pipefail
cd "$(dirname "$0")/.."

./rust-migration/build-crate.sh
./rust-migration/run-cycles.sh
