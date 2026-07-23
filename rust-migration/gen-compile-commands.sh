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

export PREMAKE5="${BINDIR}/premake5"
export BD_NINJA="${BINDIR}/ninja"
chmod +x "${PREMAKE5}" "${BD_NINJA}" 2>/dev/null || true

make -f quick.make "${NINJA_TARGET}"
( cd build/ninja && "${BD_NINJA}" -t compdb cc ) > /tmp/compdb.full.json
node rust-migration/filter-compdb.js /tmp/compdb.full.json compile_commands.json
