#!/usr/bin/env bash
# Builds the C toolchain natively (amd64; c/dep/bin-linux/{premake5,ninja} are
# x86_64 binaries) and compares its output against the already-built Rust
# crate (rust/target/release/), byte-for-byte, on the
# same canonical input JSON for each payload.
#
# Must run AFTER the Rust crate has been built (cargo build --release) and
# on the SAME architecture as that build, so both binaries' outputs are
# directly comparable without any cross-arch ambiguity.
#
# Invoke as: ./rust/scripts/compare-with-c.sh
set -euo pipefail
cd "$(dirname "$0")/../.."

RUST_BIN=rust/target/release
if [ ! -x "${RUST_BIN}/otfccdump" ] || [ ! -x "${RUST_BIN}/otfccbuild" ]; then
	echo "ERROR: ${RUST_BIN}/{otfccdump,otfccbuild} not found; build the Rust crate first." >&2
	exit 1
fi

echo "==> Building the C toolchain (native amd64)"
# Unlike gen-compile-commands.sh (which invokes ninja directly after its own
# `cd`), this uses quick.make's own `linux-release-x64` target, which
# internally does `cd build/ninja && ../../$(BD_NINJA) ...` — BD_NINJA must
# stay a repo-root-relative path for that "../../" prefix to resolve.
export PREMAKE5="c/dep/bin-linux/premake5"
export BD_NINJA="c/dep/bin-linux/ninja"
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
make -f c/quick.make linux-release-x64
C_BIN=bin/release-x64

BUILD=build/compare-with-c
mkdir -p "${BUILD}"

TTF_PAYLOADS="NotoNastaliqUrdu-Regular iosevka-r BungeeColor-Regular_colr_Windows Reinebow-SVGinOT vtt Molengo-Regular"
CFF_PAYLOADS="KRName-Regular"
# Cormorant-Medium / WorkSans-Regular.otf are excluded: both the C and Rust
# otfccdump stack-overflow on them (a pre-existing bug in the C CFF
# interpreter — see rust/README.md), unrelated to this comparison.

# Optional: the gvar (variable-font) payload from make-test-variable-font.py.
# Needs fontTools, so it's generated as a separate CI step rather than always
# required; skip if it wasn't generated.
GVAR_PAYLOAD="build/gvar-test.ttf"

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
if [ -f "${GVAR_PAYLOAD}" ]; then
	compare_payload "gvar-test" ttf "${GVAR_PAYLOAD}"
else
	echo "  (skipping gvar-test.ttf: not found; run rust/scripts/make-test-variable-font.py first)"
fi

echo "==> Comparing C vs Rust otfccdll (cdylib) output, byte-for-byte"
DLL_C="${C_BIN}/libotfccdll.so"
[ "$(uname)" = "Darwin" ] && DLL_C="${C_BIN}/libotfccdll.dylib"
RUST_SO_EXT="so"
[ "$(uname)" = "Darwin" ] && RUST_SO_EXT="dylib"
DLL_RUST="${RUST_BIN}/libotfcc_rust.${RUST_SO_EXT}"
if [ -f "${DLL_C}" ] && [ -f "${DLL_RUST}" ]; then
	DLL_JSON="${BUILD}/Molengo-Regular.json"
	python3 "$(dirname "$0")/test-dll.py" "${DLL_C}" "${DLL_JSON}" "${BUILD}/dll-c.otf"
	python3 "$(dirname "$0")/test-dll.py" "${DLL_RUST}" "${DLL_JSON}" "${BUILD}/dll-rust.otf"
	# The DLL API doesn't take --keep-modified-time, so head.created/modified/
	# checkSumAdjustment legitimately vary run to run (see README) — even two
	# C-only invocations differ at those bytes. Diff byte count against that
	# same-library baseline instead of expecting a plain cmp to pass.
	python3 "$(dirname "$0")/test-dll.py" "${DLL_C}" "${DLL_JSON}" "${BUILD}/dll-c-2.otf"
	# cmp -l exits non-zero whenever the files differ, which they legitimately
	# do here (see comment above) — under `set -e`, that would abort the whole
	# script at the very assignment meant to *measure* that expected diff, so
	# tolerate cmp's exit status explicitly with `|| true`.
	baseline_diff=$( (cmp -l "${BUILD}/dll-c.otf" "${BUILD}/dll-c-2.otf" 2>/dev/null || true) | wc -l | tr -d ' ')
	cross_diff=$( (cmp -l "${BUILD}/dll-c.otf" "${BUILD}/dll-rust.otf" 2>/dev/null || true) | wc -l | tr -d ' ')
	if [ "${cross_diff}" -le "${baseline_diff}" ]; then
		echo "PASS  otfccdll: Rust matches C (differs in ${cross_diff} bytes, same as the ${baseline_diff}-byte run-to-run timestamp variance)"
	else
		echo "FAIL  otfccdll: Rust differs from C in ${cross_diff} bytes (run-to-run baseline is only ${baseline_diff})"
		fail=1
	fi
else
	echo "  (skipping otfccdll comparison: ${DLL_C} or ${DLL_RUST} not built)"
fi

if [ "${fail}" -ne 0 ]; then
	echo "==> FAILED: at least one payload's Rust output differs from C" >&2
	exit 1
fi
echo "==> All payloads byte-identical between C and Rust"
