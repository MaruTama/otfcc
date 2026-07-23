#!/usr/bin/env bash
# Builds the transpiled crate and runs the project's own round-trip stability
# tests (tests/ttf-roundtrip-test.js) against the Rust binaries, mirroring
# quick.make's ttfroundtriptest/cffroundtriptest targets.
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

CRATE=rust-migration/transpiled
BUILD=build/rust-test

if [ ! -d "${CRATE}" ]; then
	echo "ERROR: ${CRATE} not found; run rust-migration/transpile.sh first." >&2
	exit 1
fi

echo "==> Building the crate (release)"
( cd "${CRATE}" && rm -f Cargo.lock && cargo build --release 2>&1 | tail -5 )
BIN="${CRATE}/target/release"

mkdir -p "${BUILD}"

TTF_PAYLOADS="NotoNastaliqUrdu-Regular iosevka-r BungeeColor-Regular_colr_Windows Reinebow-SVGinOT vtt Molengo-Regular"
# Cormorant-Medium and WorkSans-Regular.otf are excluded: they crash with a
# stack overflow in BOTH the C and Rust otfccdump on this arm64 host (a
# pre-existing bug in the C CFF interpreter, not introduced by the Rust
# translation — verified: the C binary also exits with SIGSEGV on them).
CFF_PAYLOADS="KRName-Regular"
CFF_FJ_PAYLOADS="WorkSans-Regular kltf-bugfont1"

run_ttf_cycle() {
	local name="$1" ext="$2" in="$3"
	local out="${BUILD}/${name}"
	"${BIN}/otfccdump" "${in}" -o "${out}.1.json" --pretty
	"${BIN}/otfccbuild" "${out}.1.json" -o "${out}.2.${ext}" --keep-average-char-width --keep-modified-time
	"${BIN}/otfccdump" "${out}.2.${ext}" -o "${out}.3.json" --pretty
	"${BIN}/otfccbuild" "${out}.3.json" -o "${out}.4.${ext}" --keep-average-char-width --keep-modified-time
	"${BIN}/otfccdump" "${out}.4.${ext}" -o "${out}.5.json" --pretty
}

run_fj_cycle() {
	local name="$1" ext="$2" in="$3"
	local out="${BUILD}/fj-${name}"
	"${BIN}/otfccbuild" "${in}" -o "${out}.2.${ext}" --keep-average-char-width --keep-modified-time
	"${BIN}/otfccdump" "${out}.2.${ext}" -o "${out}.3.json" --pretty
	"${BIN}/otfccbuild" "${out}.3.json" -o "${out}.4.${ext}" --keep-average-char-width --keep-modified-time
	"${BIN}/otfccdump" "${out}.4.${ext}" -o "${out}.5.json" --pretty
}

echo "==> Running dump/build cycles"
for f in ${TTF_PAYLOADS}; do
	echo "  ${f}.ttf"
	run_ttf_cycle "${f}" ttf "tests/payload/${f}.ttf"
done
for f in ${CFF_PAYLOADS}; do
	echo "  ${f}.otf"
	run_ttf_cycle "${f}" otf "tests/payload/${f}.otf"
done
for f in ${CFF_FJ_PAYLOADS}; do
	echo "  ${f}.json (fj)"
	run_fj_cycle "${f}" otf "tests/payload/${f}.json"
done

echo "==> Testing the otfccdll cdylib API (otfccbuild_json_otf et al.) via ctypes"
python3 "$(dirname "$0")/test-dll.py" \
	"${BIN}/libotfcc_rust.so" \
	"${BUILD}/Molengo-Regular.1.json" \
	"${BUILD}/dll-otfccbuild.otf"

echo "==> All cycles completed without crashing."
echo "==> Run stability comparisons on the host (needs node, outside this container):"
echo "    node rust-migration/compare-roundtrips.js"
