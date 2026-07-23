#!/usr/bin/env bash
# Transpiles otfcc to Rust with c2rust. Runs inside the c2rust image with the
# repo mounted at its *host* path, so the absolute paths in compile_commands.json
# (generated on the host) resolve unchanged. No premake/ninja/node needed here.
#
# Invoke as:
#   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
#       --entrypoint bash otfcc-c2rust rust-migration/transpile.sh
set -euo pipefail

DEST="rust-migration/transpiled"
# CRITICAL: the compile database must live on the *mounted* filesystem, not on
# the container's tmpfs. With the DB on /tmp, c2rust panics reliably and
# reproducibly ("Type conversion not implemented for TagTypeUnknown") partway
# through; with the byte-identical DB on the bind mount it always succeeds.
# (Likely c2rust's clang AST-exporter writes intermediates relative to the DB.)
DB="rust-migration/.compile_commands.native.json"
WORK=/tmp/otfcc-rust

if [ ! -f compile_commands.json ]; then
	echo "ERROR: compile_commands.json not found in $(pwd)." >&2
	echo "Generate it on the host first (see rust-migration/README.md)." >&2
	exit 1
fi

# The host database targets an x86_64 config (-m64). This image is native arm64
# and c2rust ignores --target/--sysroot, so it parses the native aarch64 headers;
# strip -m64 to let clang target the native arch. The image already neutralizes
# aarch64's SIMD <bits/math-vector.h>, which c2rust cannot represent. x86_64 and
# arm64 Linux are both LP64, so the transpiled Rust is equivalent.
sed 's/ -m64//g' compile_commands.json > "${DB}"

echo "==> Transpiling $(grep -c '"file"' "${DB}") translation units"
rm -rf "${WORK}"
# --emit-build-files writes Cargo.toml/build.rs; -b marks binary entrypoints.
c2rust transpile "${DB}" \
	--emit-build-files \
	--output-dir "${WORK}" \
	-b otfccdump \
	-b otfccbuild
# Note: otfccdll.c is the shared-library entry (no main), so it is NOT a -b
# binary target; it is compiled into the library.

# --- Post-transpile fixups so the crate compiles (build with the pinned
# nightly in rust-toolchain.toml; c2rust output needs release / overflow-checks
# off at runtime) ---
# c2rust emits `let ref mut freshN = (*packed).len;` which takes a reference to
# a packed-struct field — a hard error (E0793). Rewrite to a raw pointer.
sed -i 's/let ref mut \(fresh[0-9]*\) =/let \1 = \&raw mut/' \
	"${WORK}/src/dep/extern/sds.rs"

# Runtime: the vendored dtoa indexes kPow10 (a [u32; 10]) with -kappa, which can
# reach 12 for high-precision doubles. In C that reads adjacent memory (a large
# value that makes GrisuRound a no-op); Rust bounds-checks and panics. Clamp the
# index to the array bounds, reproducing C's effective (no-op-rounding) behavior.
sed -i 's/kPow10\[-kappa as usize\]/kPow10[(-kappa as usize).min(9)]/; s/kPow10\[kappa as usize\]/kPow10[(kappa as usize).min(9)]/' \
	"${WORK}/src/dep/extern/emyg_dtoa/emyg_dtoa.rs"

echo "==> Copying finished crate to ${DEST}"
rm -rf "${DEST}"
mkdir -p "$(dirname "${DEST}")"
cp -r "${WORK}" "${DEST}"
rm -f "${DB}"

echo "==> Transpile complete. Crate at ${DEST}"
echo "    modules: $(find "${DEST}" -name '*.rs' | wc -l)"
