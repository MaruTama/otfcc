#!/usr/bin/env bash
# Generate compile_commands.json on the host as c2rust input.
# Produces the single release-x64 C config (118 translation units).
#
# macOS:  ./rust-migration/gen-compile-commands.sh
# Linux:  OS=linux ./rust-migration/gen-compile-commands.sh
set -euo pipefail
cd "$(dirname "$0")/.."

OS="${OS:-macosx}"
case "${OS}" in
	macosx) BINDIR=dep/bin-osx;   NINJA_TARGET=mf-ninja-macosx ;;
	linux)  BINDIR=dep/bin-linux; NINJA_TARGET=mf-ninja-linux  ;;
	*) echo "Unknown OS=${OS} (use macosx|linux)" >&2; exit 1 ;;
esac

# Absolute paths: BD_NINJA is invoked after `cd build/ninja` below, so a
# relative path would resolve against the wrong directory.
export PREMAKE5="$(pwd)/${BINDIR}/premake5"
export BD_NINJA="$(pwd)/${BINDIR}/ninja"
chmod +x "${PREMAKE5}" "${BD_NINJA}" 2>/dev/null || true

# quick.make's mf-ninja-linux target passes --cc=$(CC) to premake5, which
# needs an actual compiler name (gcc/clang) — Make's built-in default
# ($(CC) = "cc") isn't one, and premake5 rejects it ("invalid value 'cc' for
# option 'cc'"). Only relevant for OS=linux; the macOS path doesn't pass --cc.
if [ "${OS}" = "linux" ]; then
	export CC="${CC:-gcc}"
	if [ "${CC}" = "cc" ]; then export CC=gcc; fi
fi

make -f quick.make "${NINJA_TARGET}"
( cd build/ninja && "${BD_NINJA}" -t compdb cc ) > /tmp/compdb.full.json
node rust-migration/filter-compdb.js /tmp/compdb.full.json compile_commands.json
