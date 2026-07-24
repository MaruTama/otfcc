#!/usr/bin/env bash
# Builds the C toolchain natively (amd64; dep/bin-linux/{premake5,ninja} are
# x86_64 binaries) and compares its output against the already-built Rust
# crate (rust-migration/transpiled/target/release/), byte-for-byte, on the
# same canonical input JSON for each payload.
#
# Must run AFTER the Rust crate has been built (cargo build --release) and
# on the SAME architecture as that build, so both binaries' outputs are
# directly comparable without any cross-arch ambiguity.
#
# Invoke as: ./rust-migration/compare-with-c.sh
set -euo pipefail
cd "$(dirname "$0")/.."

RUST_BIN=rust-migration/transpiled/target/release
if [ ! -x "${RUST_BIN}/otfccdump" ] || [ ! -x "${RUST_BIN}/otfccbuild" ]; then
	echo "ERROR: ${RUST_BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

echo "==> Building the C toolchain (native amd64)"
# Unlike gen-compile-commands.sh (which invokes ninja directly after its own
# `cd`), this uses quick.make's own `linux-release-x64` target, which
# internally does `cd build/ninja && ../../$(BD_NINJA) ...` — BD_NINJA must
# stay a repo-root-relative path for that "../../" prefix to resolve.
export PREMAKE5="dep/bin-linux/premake5"
export BD_NINJA="dep/bin-linux/ninja"
chmod +x "${PREMAKE5}" "${BD_NINJA}"
# quick.make's mf-ninja-linux passes --cc=$(CC) to premake5; Make's built-in
# default ($(CC) = "cc") isn't a valid compiler name for it. Default to
# clang, not gcc: c2rust's transpile is based on parsing with clang's AST, and
# gcc vs clang produce measurably different floating-point rounding in this
# codebase (verified: a gcc-built otfccbuild differs byte-for-byte from a
# clang-built one on the SAME source and SAME machine, while clang builds
# match across architectures/OSes) — using gcc here would flag that
# gcc/clang difference as a false Rust-vs-C mismatch.
export CC="${CC:-clang}"
if [ "${CC}" = "cc" ]; then export CC=clang; fi
make -f quick.make linux-release-x64
C_BIN=bin/release-x64

BUILD=build/compare-with-c
mkdir -p "${BUILD}"

TTF_PAYLOADS="NotoNastaliqUrdu-Regular iosevka-r BungeeColor-Regular_colr_Windows Reinebow-SVGinOT vtt Molengo-Regular"
CFF_PAYLOADS="KRName-Regular"
# Cormorant-Medium / WorkSans-Regular.otf are excluded: both the C and Rust
# otfccdump stack-overflow on them (a pre-existing bug in the C CFF
# interpreter — see rust-migration/README.md), unrelated to this comparison.

fail=0

compare_payload() {
	local name="$1" ext="$2" in="$3"
	local out="${BUILD}/${name}"

	# Canonical input JSON, dumped once with the C toolchain so both builds
	# start from byte-identical input.
	"${C_BIN}/otfccdump" "${in}" -o "${out}.json" --pretty

	"${C_BIN}/otfccbuild" "${out}.json" -o "${out}.c.${ext}" --keep-average-char-width --keep-modified-time
	"${RUST_BIN}/otfccbuild" "${out}.json" -o "${out}.rust.${ext}" --keep-average-char-width --keep-modified-time

	if cmp -s "${out}.c.${ext}" "${out}.rust.${ext}"; then
		echo "PASS  ${name}.${ext}: byte-identical"
	else
		echo "FAIL  ${name}.${ext}: differs ($(cmp -l "${out}.c.${ext}" "${out}.rust.${ext}" 2>/dev/null | wc -l) bytes)"
		fail=1
	fi
}

echo "==> Comparing C vs Rust otfccbuild output, byte-for-byte"
for f in ${TTF_PAYLOADS}; do
	compare_payload "${f}" ttf "tests/payload/${f}.ttf"
done
for f in ${CFF_PAYLOADS}; do
	compare_payload "${f}" otf "tests/payload/${f}.otf"
done

if [ "${fail}" -ne 0 ]; then
	echo "==> FAILED: at least one payload's Rust output differs from C" >&2
	exit 1
fi
echo "==> All payloads byte-identical between C and Rust"
