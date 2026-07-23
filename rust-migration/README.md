# otfcc → Rust migration (c2rust) — Phase 1

This directory holds the tooling for the c2rust-based Rust migration tracked in
[issue #2](https://github.com/MaruTama/otfcc/issues/2). Phase 1 produces a
**baseline transpilation**: an unsafe, non-idiomatic Rust port that builds and
passes the existing round-trip tests. Idiomatic refactoring is Phase 2.

The C-side fix for [issue #1](https://github.com/MaruTama/otfcc/issues/1)
(large `gsub_alternate` corruption) landed separately and is carried into the
transpiled code automatically, since it is part of the C sources being
translated.

## Why native arm64, and this base image

c2rust is best supported on Linux with a known LLVM, but building it from a
stock, emulated `linux/amd64` image proved brittle: an amd64 image under QEMU
hits a fatal GNU Make jobserver bug ("write jobserver: Bad file descriptor")
building c2rust's vendored tinycbor. The image therefore targets the host's
**native arm64** instead, which avoids emulation entirely.

c2rust also cannot represent the SIMD vector-math declarations in aarch64
glibc's `<bits/math-vector.h>` (`_ZGVnN..` libm variants) and panics ("Could
not find CTypeId … in TypedAstContext"); c2rust's frontend ignores
`--target`/`--sysroot`, so it always parses the native aarch64 headers. otfcc
never uses vectorized libm, so the image replaces that header with glibc's
empty SIMD-decl stubs (`#include <bits/libm-simd-decl-stubs.h>`), which still
defines the macros `mathcalls.h` needs.

The image is Ubuntu 24.04 (LLVM/Clang 17) with **c2rust 0.22.1** — clang-18
tripped an unrelated c2rust ast-exporter bug on some ordinary code, clang-17
didn't.

> If the sandboxed shell makes `docker` hang on "load metadata" (credential
> helper), export `DOCKER_HOST=unix://$HOME/.docker/run/docker.sock` and
> `DOCKER_CONFIG=<dir with config.json = {}>` to bypass the helper for
> anonymous pulls.

## Pipeline pieces

- `gen-compile-commands.sh` / `filter-compdb.js` — host-side: premake →
  `ninja -t compdb cc` → reduce to the single release-x64 C config
  (118 translation units).
- `Dockerfile` — native-arm64 c2rust 0.22.1 image, as above.
- `transpile.sh` — runs c2rust and applies the two fixups below.
- `fix-transmute-abi.py` — c2rust sometimes wraps a zero-arg,
  struct-returning function-pointer-field call in a `transmute` that drops
  the `unsafe extern "C"` ABI when the C code assigns the result through an
  outer typedef-alias cast. This corrupted every such struct-by-value return
  (observed: a Handle's `name` field ending up holding an unrelated
  function's address → `free(): invalid pointer`). Strips the transmute,
  calling the function pointer directly.
- `fix-float-narrowing.py` — c2rust mistranslates C's *implicit*
  `pos_t` (f64) → `uintN_t` narrowing conversion at `bufwriteNNb()` call
  sites lacking an explicit intermediate cast in the C source. It emits
  `x as uint16_t`, using Rust's *saturating* float→unsigned semantics
  (negative → 0) instead of C's actual behavior (convert through a signed
  int, reinterpret the bits: `-41.0 → 0xFFD7`, decodes back to -41). This
  silently zeroed negative `hmtx.lsb` / `vmtx.tsb` /
  `VORG.defaultVerticalOrigin` in the built font. Fixed at the 5 confirmed
  call sites.
- `test.sh` — builds the transpiled crate and runs the same dump/build cycles
  as `quick.make`'s round-trip targets, against every payload the C test
  suite covers (minus two fonts that crash both C and Rust with a stack
  overflow — see Status below).
- `compare-roundtrips.js` — runs `tests/ttf-roundtrip-test.js` over every
  payload `test.sh` produced and reports a single pass/fail summary.
- `make-test-variable-font.py` — builds a minimal, self-contained variable
  font (fvar + gvar, one `wght` axis, two masters, via fontTools APIs — no
  external download) to exercise the gvar delta-application path, which none
  of `tests/payload/*.ttf` has.
- `test-dll.py` — exercises the `otfccdll` C API (`otfccbuild_json_otf` /
  `otfcc_get_buf_len` / `otfcc_get_buf_data` / `otfccbuild_free_otfbuf`) via
  `ctypes`, against either the C `libotfccdll.{dylib,so}` or the Rust
  `cdylib`, to compare output byte-for-byte.

Generated artifacts (`compile_commands.json`, `rust-migration/transpiled/`)
are gitignored — nothing generated is committed.

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

3. Transpile. The repo is mounted at its **host path** so the absolute paths
   in `compile_commands.json` resolve unchanged:

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

## Status: Phase 1 complete

The pipeline transpiles **all 118 translation units** to Rust in one pass,
emitting a full Cargo crate (`Cargo.toml`, `lib.rs`, `build.rs`, the
`otfccdump`/`otfccbuild` binaries — `otfccdll` compiles into the lib since it
has no `main`).

The crate **builds and its round-trips are byte-for-byte correct**:

- The Rust binaries pass `tests/ttf-roundtrip-test.js` on all 6 TTF payloads,
  the CFF payload `KRName-Regular`, and both from-JSON CFF payloads.
- Building the *same* input JSON with the C toolchain and the Rust toolchain
  produces `.ttf`/`.otf` files that are **byte-identical** (`cmp` shows 0
  differing bytes) for all 7 directly-comparable payloads.
- **Variable-font (gvar) coverage**: none of `tests/payload/*.ttf` has an
  `fvar` table, so the gvar delta-application path (`applyPolymorphism` in
  `lib/table/glyf/read.c`) was untested by the payload matrix above.
  `make-test-variable-font.py` closes that gap — verified C and Rust are
  byte-identical at every stage of a full two-cycle round trip (original
  dump, build 1, post-build dump 1, build 2, post-build dump 2). This also
  surfaced a pre-existing otfcc limitation (`otfccbuild` doesn't reconstruct
  `fvar`/`gvar` from JSON with delta-annotated coordinates), but it
  reproduces identically in C and Rust, confirming it's an existing gap in
  otfcc's build-side variable-font support, not a migration regression.

Two fonts (`Cormorant-Medium.otf`, `WorkSans-Regular.otf`) crash both the C
*and* Rust `otfccdump` on this arm64 host with a stack overflow — a
pre-existing bug in the C CFF interpreter (verified: the C binary also exits
SIGSEGV on them), not something the Rust translation introduced or needs to
fix here.

**`otfccdll` (cdylib) coverage**: the C build (`premake5.lua`) builds
`otfccdll.c`'s exported functions as a `SharedLib`; `transpile.sh` adds
`cdylib` to the crate's `crate-type` so the same `#[no_mangle] extern "C"`
functions are callable the same way. Verified via `test-dll.py` (ctypes):
calling `otfccbuild_json_otf` through the Rust `.so` and the C
`.dylib`/`.so` on the same JSON input produces output that's byte-identical
except at the 3 bytes that also differ between two separate invocations of
the *C* library alone (the DLL API doesn't accept `--keep-modified-time`, so
`head.created`/`modified`/`checkSumAdjustment` legitimately vary run to run)
— i.e. functionally identical.

> Non-obvious gotcha (handled by `transpile.sh`): the compile database
> **must** live on the mounted filesystem, not the container's `/tmp`. With
> the byte-identical DB on `/tmp`, c2rust panics reliably ("Type conversion
> not implemented for TagTypeUnknown"); on the bind mount it always
> succeeds. Likely c2rust's clang AST-exporter writes intermediates relative
> to the DB. Also build with the **pinned nightly** in `rust-toolchain.toml`,
> not `cargo +nightly` (the image's latest nightly) — c2rust's va_list output
> only matches the pinned toolchain's API.

## Next steps (Phase 2: idiomatization)

- Wire steps 1–5 into CI alongside the existing C build.
- Begin replacing `unsafe`, macro-expanded, C-shaped code with idiomatic Rust,
  module by module, keeping the round-trip tests green throughout.
