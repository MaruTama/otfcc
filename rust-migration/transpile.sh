#!/usr/bin/env bash
# Transpiles otfcc to Rust with c2rust. Runs inside the c2rust image with the
# repo mounted at its *host* path, so the absolute paths in compile_commands.json
# (generated on the host) resolve unchanged. No premake/ninja/node needed here.
#
# Invoke as:
#   docker run --rm --platform linux/amd64 -v "$PWD":"$PWD" -w "$PWD" \
#       --entrypoint bash otfcc-c2rust rust-migration/transpile.sh
set -euo pipefail

OUT="rust-migration/transpiled"

if [ ! -f compile_commands.json ]; then
	echo "ERROR: compile_commands.json not found in $(pwd)." >&2
	echo "Generate it on the host first (see rust-migration/README.md)." >&2
	exit 1
fi

# The host database was produced for an x86_64 config (-m64). This image is
# native arm64 (c2rust ignores --target/--sysroot, so it parses the native
# aarch64 headers), so strip -m64 to let clang target the native arch. The
# image already neutralizes aarch64's SIMD <bits/math-vector.h>, which c2rust
# cannot represent. x86_64 and arm64 Linux are both LP64, so the Rust matches.
sed 's/ -m64//g' compile_commands.json > /tmp/compile_commands.native.json

echo "==> Transpiling $(grep -c '"file"' /tmp/compile_commands.native.json) translation units (native aarch64)"
rm -rf "${OUT}"

# --emit-build-files writes Cargo.toml/build.rs; -b marks binary entrypoints.
c2rust transpile /tmp/compile_commands.native.json \
	--emit-build-files \
	--output-dir "${OUT}" \
	-b otfccdump \
	-b otfccbuild \
	-b otfccdll

echo "==> Transpile complete. Output in ${OUT}"
ls -la "${OUT}" | head -40
