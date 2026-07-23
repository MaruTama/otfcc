# otfcc → Rust migration (c2rust) — Phase 1

This directory holds the tooling for the c2rust-based Rust migration tracked in
[issue #2](https://github.com/MaruTama/otfcc/issues/2). Phase 1 produces a
**baseline transpilation**: an unsafe, non-idiomatic Rust port that builds and
passes the existing round-trip tests. Idiomatic refactoring is Phase 2.

The C-side fix for [issue #1](https://github.com/MaruTama/otfcc/issues/1)
(large `gsub_alternate` corruption) landed separately and is carried into the
transpiled code automatically, since it is part of the C sources being
translated.

## Why Docker / Linux, and this base image

c2rust is best supported on Linux with a known LLVM. Building it from a stock
Ubuntu proved brittle (Clang AST-API drift, old lockfile vs new rustc), so the
image is based on the **c2rust project's own CI image**
`immunant/c2rust:ubuntu-focal-latest` (Ubuntu 20.04, LLVM 10,
rust `nightly-2022-08-08`) and just installs the matching **c2rust 0.16.0**.

The image targets `linux/amd64`: the base is amd64 and the compilation database
targets `x86_64` (`-m64`), keeping the transpiled Rust consistent with the C
build.

> If the sandboxed shell makes `docker` hang on "load metadata" (credential
> helper), export `DOCKER_HOST=unix://$HOME/.docker/run/docker.sock` and
> `DOCKER_CONFIG=<dir with config.json = {}>` to bypass the helper for
> anonymous pulls.

## Usage

1. Generate the compilation database on the host (macOS shown; `OS=linux` on Linux):

   ```bash
   ./rust-migration/gen-compile-commands.sh
   ```

   This writes `compile_commands.json` (118 release-x64 translation units).

2. Build the transpiler image once (native arm64; slow — it compiles c2rust
   from source):

   ```bash
   docker build -t otfcc-c2rust -f rust-migration/Dockerfile rust-migration/
   ```

3. Run the transpile. The repo is mounted at its **host path** so the absolute
   paths in `compile_commands.json` resolve unchanged:

   ```bash
   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
       --entrypoint bash otfcc-c2rust rust-migration/transpile.sh
   ```

Output lands in `rust-migration/transpiled/` (a Cargo project with `Cargo.toml`,
`build.rs`, and one Rust module per C translation unit). It is generated,
non-idiomatic, and **not** committed.

## Pipeline pieces

- `gen-compile-commands.sh` — host: premake → `ninja -t compdb cc` → filter.
- `filter-compdb.js` — reduce to the release-x64 C config.
- `Dockerfile` — c2rust 0.16.0 on the project's known-good CI toolchain.
- `transpile.sh` — in-container: `c2rust transpile … -b otfccdump -b otfccbuild -b otfccdll`.

## Status: Phase 1 complete

The pipeline transpiles **all 118 translation units** to Rust in one pass with
c2rust 0.22.1 on clang-17, emitting a full Cargo crate (`Cargo.toml`, `lib.rs`,
`build.rs`, the `otfccdump`/`otfccbuild` binaries — `otfccdll` compiles into
the lib since it has no `main`).

The crate **builds and its round-trips are byte-for-byte correct**: the Rust
binaries pass the project's own `tests/ttf-roundtrip-test.js` on all 6 TTF
payloads, the CFF payload `KRName-Regular`, and both from-JSON CFF payloads.
`rust-migration/test.sh` + `compare-roundtrips.js` reproduce this from a clean
transpile.

Two fonts (`Cormorant-Medium.otf`, `WorkSans-Regular.otf`) crash both the C
*and* Rust `otfccdump` on this arm64 host with a stack overflow — a
pre-existing bug in the C CFF interpreter (verified: the C binary also exits
SIGSEGV on them), not something the Rust translation introduced or needs to
fix here.

> Non-obvious gotchas (all handled by `transpile.sh`):
> - The compile database **must** live on the mounted filesystem, not the
>   container's `/tmp`. With the byte-identical DB on `/tmp`, c2rust panics
>   reliably ("Type conversion not implemented for TagTypeUnknown"); on the
>   bind mount it always succeeds.
> - Build with the **pinned nightly** in `rust-toolchain.toml`, not
>   `cargo +nightly` (the image's latest nightly) — c2rust's va_list output
>   only matches the pinned toolchain's API.
> - c2rust sometimes drops the `unsafe extern "C"` ABI on struct-returning
>   function-pointer calls (`fix-transmute-abi.py`) and mistranslates
>   `pos_t -> uintN_t` implicit narrowing casts (`fix-float-narrowing.py`),
>   both silently corrupting output without crashing. See the two scripts for
>   the exact mechanism and how each was found.

## Usage

1. Generate the compilation database on the host (macOS shown; `OS=linux` on Linux):

   ```bash
   ./rust-migration/gen-compile-commands.sh
   ```

2. Build the transpiler image once (native arm64; slow — it compiles c2rust
   from source):

   ```bash
   docker build -t otfcc-c2rust -f rust-migration/Dockerfile rust-migration/
   ```

3. Transpile:

   ```bash
   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
       --entrypoint bash otfcc-c2rust rust-migration/transpile.sh
   ```

4. Build the crate and run the dump/build cycles:

   ```bash
   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
       --entrypoint bash otfcc-c2rust rust-migration/test.sh
   ```

5. Compare round-trip stability on the host (needs `node`):

   ```bash
   node rust-migration/compare-roundtrips.js
   ```

## Variable-font (gvar) coverage

None of `tests/payload/*.ttf` has an `fvar` table, so the gvar delta-application
path (`applyPolymorphism` in `lib/table/glyf/read.c`) was completely untested
by the payload matrix above. `rust-migration/make-test-variable-font.py`
builds a minimal self-contained variable font (one `wght` axis, two masters,
via fontTools — no external download) to close that gap.

Result: **C and Rust are byte-identical at every stage** of a full two-cycle
round trip (original dump, first build, post-build dump, second build,
second post-build dump) — including a pre-existing otfcc limitation this
test surfaced: `otfccbuild` does not reconstruct `fvar`/`gvar` from JSON
containing delta-annotated coordinates (the variable dimension is lost after
one build, and the affected coordinates collapse to 0). This reproduces
**identically** in C and Rust (same bytes), so it's a gap in otfcc's existing
build-side variable-font support, not a migration regression — out of scope
to fix here.

## Next steps (Phase 2: idiomatization)

- Wire steps 1–5 into CI alongside the existing C build.
- Test against `otfccdll` payloads (compiles into the lib but untested as a
  cdylib).
- Begin replacing `unsafe`, macro-expanded, C-shaped code with idiomatic Rust,
  module by module, keeping the round-trip tests green throughout.
