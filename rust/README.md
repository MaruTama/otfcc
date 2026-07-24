# otfcc → Rust migration (c2rust)

This directory holds the Rust port of otfcc, produced by transpiling the C
sources with [c2rust](https://c2rust.com/) and tracked in
[issue #2](https://github.com/MaruTama/otfcc/issues/2).

**`rust/` is committed source, not a build artifact.**
The plan is to eventually delete the C implementation and let this Rust code
be the real, standalone implementation — so unlike a typical generated-code
directory, it's checked into the repo and (from Phase 2 onward) will be
hand-edited directly. Only `rust/target/` (the actual
compiled binaries) is gitignored.

The C-side fix for [issue #1](https://github.com/MaruTama/otfcc/issues/1)
(large `gsub_alternate` corruption) landed separately and is carried into the
committed Rust source, since it was part of the C sources at transpile time.

**Repository layout**: the C implementation (`lib/`, `src/`, `dep/`,
`include/`, `premake5.lua`, `quick.make`, `_vc*.bat`) lives under `c/` at the
repo root, mirroring how the Rust port is self-contained under
`rust/`. This is deliberate: once Rust is trusted as the sole
implementation, retiring C is a single `rm -rf c/` (see "Next steps" below).
`tests/`, `build/`, and `bin/` stay at the repo root — both sides build
against and are verified against those same shared fixtures/outputs, so
splitting them would mean duplicating the font payloads. `c/premake5.lua`
and `c/quick.make` are written to still produce `build/` and `bin/` at the
repo root (not nested under `c/`), so nothing downstream (this directory's
scripts, CI) had to change its output-path assumptions. This directory is
flattened: the crate root (`Cargo.toml`, `lib.rs`, `src/`) and the migration
tooling (`compare-with-c.sh`, `Dockerfile`, this README, ...) live side by
side directly in `rust/` — there is no separate `transpiled/` subdirectory.
`transpile.sh` knows the difference and only touches the crate-owned files
when regenerating (see "Regenerating the Rust source" below).

## Everyday use: just build and test

CI (`.github/workflows/rust.yml`) and local development do **not**
re-run c2rust. They just build the committed Rust source and check it:

```bash
pip install fonttools && python3 rust/scripts/make-test-variable-font.py
                                   # optional: adds the gvar payload below
./rust/scripts/build-crate.sh   # cargo build --release + cargo test
./rust/scripts/compare-with-c.sh # build C with clang, compare byte-for-byte
./rust/scripts/run-cycles.sh    # dump/build cycles against the Rust binaries
node rust/scripts/compare-roundtrips.js
```

(`./rust/scripts/test.sh` = `build-crate.sh` + `run-cycles.sh`, for
convenience.) None of this needs Docker, c2rust, or a specific architecture —
plain `rustup`/`cargo` plus a C compiler.

## Regenerating the Rust source (manual only, and now mostly historical)

Re-transpiling is a **manual, local, occasional** step — done after a C-side
change, reviewed like any other diff, and committed. It is not automated in
CI. Requires Docker and (see below) a **native arm64** host.

**Since Phase 2 (idiomatization) started, re-running this will destroy hand-
edited code.** `transpile.sh` replaces the c2rust-owned crate files under
`rust/` (`src/`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `lib.rs` — not the
hand-maintained scripts/Dockerfile/README that now share this flattened
directory) with fresh c2rust output, which would overwrite every idiomatized
file (`lib/support/buffer/buffer.rs`, `lib/bk/bkblock.rs`,
`lib/bk/bkgraph.rs`, `lib/support/alloc.rs`, `lib.rs`'s module list, ...).
The steps below are kept for reference/audit purposes and for the C-only
files that haven't been idiomatized yet, but from here on a C-side change
should be ported to the Rust side by hand (mirroring whatever the equivalent
C diff does), not by re-transpiling.

1. Generate the compilation database (macOS shown; `OS=linux` on Linux):

   ```bash
   ./rust/scripts/gen-compile-commands.sh
   ```

2. Build the transpiler image once (native arm64; slow — it compiles c2rust
   from source):

   ```bash
   docker build -t otfcc-c2rust -f rust/scripts/Dockerfile rust/scripts/
   ```

3. Transpile. The repo is mounted at its **host path** so the absolute paths
   in `c/compile_commands.json` resolve unchanged. This overwrites the crate
   files under `rust/` (not its hand-maintained scripts) — review the diff
   before committing.

   ```bash
   docker run --rm -v "$PWD":"$PWD" -w "$PWD" \
       --entrypoint bash otfcc-c2rust rust/scripts/transpile.sh
   ```

4. Verify it still builds and matches C (see "Everyday use" above), then
   commit the diff.

### Why native arm64, and this base image

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

Once transpiled, though, the resulting Rust source is **not**
architecture-specific: building the arm64-transpiled source on amd64
(verified) produces output byte-identical to a same-machine C build. Only the
transpile step itself needs arm64.

> If the sandboxed shell makes `docker` hang on "load metadata" (credential
> helper), export `DOCKER_HOST=unix://$HOME/.docker/run/docker.sock` and
> `DOCKER_CONFIG=<dir with config.json = {}>` to bypass the helper for
> anonymous pulls.

### Post-transpile fixups (applied automatically by `transpile.sh`)

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
- Also adds `cdylib` to the crate's `crate-type` (the C build makes
  `otfccdll.c` a `SharedLib`; c2rust's default `staticlib`+`rlib` alone can't
  be linked against as a shared library) and clamps the vendored dtoa's
  `kPow10` index (a latent OOB in the C source that Rust's bounds checks
  catch — see the comment in `transpile.sh` for the full mechanism).

## Pipeline pieces

- `gen-compile-commands.sh` / `filter-compdb.js` — host-side: premake →
  `ninja -t compdb cc` → reduce to the single release-x64 C config
  (118 translation units). Only needed to regenerate the Rust source.
- `Dockerfile` — native-arm64 c2rust 0.22.1 image. Only needed to regenerate.
- `transpile.sh` — runs c2rust and the fixups above. Only needed to regenerate.
- `build-crate.sh` — builds the committed crate (release) and runs
  `cargo test`. Needs only rustup + cargo (the pinned nightly in
  `rust-toolchain.toml`) — no c2rust/Docker, works on any architecture.
- `run-cycles.sh` — runs the same dump/build cycles as `c/quick.make`'s
  round-trip targets against an already-built crate, for every payload the C
  test suite covers (minus two fonts that crash both C and Rust with a stack
  overflow — see Status below), plus the `otfccdll` cdylib test if built.
- `test.sh` — convenience wrapper: `build-crate.sh` + `run-cycles.sh`.
- `compare-roundtrips.js` — runs `tests/ttf-roundtrip-test.js` over every
  payload produced and reports a single pass/fail summary.
- `compare-with-c.sh` — builds the C toolchain **with clang** and compares
  its output against an already-built Rust crate byte-for-byte, on the same
  machine. Defaults to clang, not gcc: c2rust's transpile is based on parsing
  with clang's AST, and gcc vs clang produce measurably different
  floating-point rounding in this codebase (verified: a gcc build differs
  byte-for-byte from a clang build of the *identical* source on the
  *identical* machine, while clang builds match byte-for-byte across
  architectures and OSes — arm64 Linux, amd64 Linux, and arm64 macOS all
  agree). Using gcc here would flag that gcc/clang difference as a false
  Rust-vs-C mismatch.
- `make-test-variable-font.py` — builds a minimal, self-contained variable
  font (fvar + gvar, one `wght` axis, two masters, via fontTools APIs — no
  external download) to exercise the gvar delta-application path, which none
  of `tests/payload/*.ttf` has. Needs `fontTools` (`pip install fonttools`);
  writes `build/gvar-test.ttf`. CI generates this before every run; locally,
  `compare-with-c.sh`/`run-cycles.sh` pick it up automatically if present and
  skip it (with a message) otherwise.
- `test-dll.py` — exercises the `otfccdll` C API (`otfccbuild_json_otf` /
  `otfcc_get_buf_len` / `otfcc_get_buf_data` / `otfccbuild_free_otfbuf`) via
  `ctypes`, against either the C `libotfccdll.{dylib,so}` or the Rust
  `cdylib`, to compare output byte-for-byte. `compare-with-c.sh` runs this
  against both libraries on the same JSON input and diffs the result.

## Status: Phase 1 complete

The committed crate (`Cargo.toml`, `lib.rs`, `build.rs`, the
`otfccdump`/`otfccbuild` binaries, `otfccdll` compiled into the lib) **builds
and its round-trips are byte-for-byte correct**:

- The Rust binaries pass `tests/ttf-roundtrip-test.js` on all 6 TTF payloads,
  the CFF payload `KRName-Regular`, both from-JSON CFF payloads, and the
  generated gvar payload (see below) — 10 payloads total.
- Building the *same* input JSON with the C toolchain and the Rust toolchain
  produces `.ttf`/`.otf` files that are **byte-identical** (`cmp` shows 0
  differing bytes) for all 8 directly-comparable payloads.
- **Variable-font (gvar) coverage**: none of `tests/payload/*.ttf` has an
  `fvar` table, so the gvar delta-application path (`applyPolymorphism` in
  `c/lib/table/glyf/read.c`) was untested by the payload matrix above.
  `make-test-variable-font.py` closes that gap — CI generates the font fresh
  every run and it's part of the regular byte-comparison and round-trip
  matrix. C and Rust are byte-identical at every stage of a full two-cycle
  round trip (original dump, build 1, post-build dump 1, build 2, post-build
  dump 2). This also surfaced a pre-existing otfcc limitation (`otfccbuild`
  doesn't reconstruct `fvar`/`gvar` from JSON with delta-annotated
  coordinates), but it reproduces identically in C and Rust, confirming it's
  an existing gap in otfcc's build-side variable-font support, not a
  migration regression.

Two fonts (`Cormorant-Medium.otf`, `WorkSans-Regular.otf`) crash both the C
*and* Rust `otfccdump` with a stack overflow — a pre-existing bug in the C
CFF interpreter (verified: the C binary also exits SIGSEGV on them), not
something the Rust translation introduced or needs to fix here.

**CI checks four things**: the crate builds and `cargo test` passes
(currently 0 tests — c2rust generates none; Phase 2 is where real coverage
gets added), its output (including the gvar payload and the `otfccdll`
cdylib) is byte-identical to the C toolchain's (`compare-with-c.sh`), and the
round-trip stability tests pass (`compare-roundtrips.js`). It's a single
`ubuntu-latest` job, no Docker.

**`otfccdll` (cdylib) coverage**: `compare-with-c.sh` calls
`otfccbuild_json_otf` through both the Rust `.so` and the C `.so`/`.dylib`
(via `test-dll.py`/ctypes) on the same JSON input and diffs the results. The
DLL API doesn't accept `--keep-modified-time`, so
`head.created`/`modified`/`checkSumAdjustment` legitimately vary run to run
even between two C-only invocations — the check compares the Rust-vs-C byte
diff against that same-library run-to-run baseline instead of requiring a
plain `cmp` pass.

## Status: Phase 2 in progress (idiomatization)

The crate went from raw c2rust output (317 compiler warnings, exclusively
`unsafe extern "C" fn`s full of manual pointer-offset loops and redundant
casts) to a **0-warning build** with `lib/support/buffer/buffer.rs`,
`lib/bk/bkblock.rs`, and `lib/bk/bkgraph.rs` (the "support/low-level I/O"
layer the original plan called out as the correct starting point — it's the
most-depended-on code and the least entangled with the rest of the crate)
rewritten to idiomatic bodies:

- **Crate-wide, byte-preserving mechanical sweep**: `cargo fix` removed 151
  unused-variable and 50 unnecessary-parens warnings; a script-driven pass
  stripped 113 redundant `unsafe` blocks (struct-of-fn-pointer vtable
  initializers that never needed the keyword); another replaced all 434
  `(true|false)_0 != 0` occurrences (c2rust's rename of C's `true`/`false`
  macros, since they collide with Rust keywords) with plain `true`/`false`.
- **buffer.rs**: the eight `bufwrite16l/b`..`bufwrite64l/b` functions'
  manual per-byte shift/mask/store expansions became
  `x.to_le_bytes()`/`to_be_bytes()` through one shared `buf_push_bytes()`
  helper; `bufwrite_sds/_str/_bytes/_buf/_bufdel` build a slice and go
  through the same helper instead of a raw `memcpy` call; index `while`
  loops became `for` loops throughout.
- **bkblock.rs** / **bkgraph.rs**: triple-cast type comparisons
  (`x as c_uint == Y as c_int as c_uint`, where both sides were already the
  same type) became `match`/`==` on the named `bk_CellType` consts directly;
  every index `while` loop became a `for` loop, including
  `dfs_attract_cells`'s reverse-iteration underflow-sentinel trick
  (`j = length; loop { let fresh = j; j -= 1; if fresh == 0 { break } ...
  }`), which became `for j in (0..length).rev()`; the offsets-prefix-sum
  computation duplicated identically in three functions was factored into
  one private `compute_block_offsets()`. **The actual offset-overflow
  decision logic — `getoffset()`, `getoffset_untangle()`, and
  `try_untabgle_block`'s bounds check — was deliberately left untouched
  beyond an equivalent-by-construction rewrite** (`offset < 0 || offset >
  0xffff` → `!(0..=0xffff).contains(&offset)`), since this module is where
  issue #1's fix lives; verified with `tests/gsub-alternate-large-test.js`
  (the dedicated issue #1 regression test) run directly against the Rust
  build, not just the standard payload matrix.
- **alloc.rs** (new): `__caryll_allocate_clean`/`__caryll_reallocate` were
  duplicated byte-for-byte in every file that used them (c2rust's per-
  translation-unit expansion of a `static inline` C header); factored out to
  one shared module and wired into the three files above. Neither helper was
  ever `#[no_mangle]`, so this changes no ABI. The ~47 other files with their
  own copy (and the still-per-file `read_8u/16u/24u/32u` family, unused in
  support/bk) are unchanged — rolling this out further is a separate,
  larger, more carefully-reviewed pass.

- **`otf_reader`/`otf_writer`** (`otf_reader.rs`, `unconsolidate.rs`,
  `otf_writer.rs`, `stat.rs`): the second module idiomatized, after
  support/bk. Two functions (`decideFontSubtypeOTF`,
  `statMaxContextOTL`) turned out to be c2rust's translation of otfcc's own
  `foreach(item, vector) { ... }` macro — `__fortable_*`/`__caryll_index*`/
  `keep*` variables simulating a single-iteration inner loop purely so the
  macro body can `break`/`continue`; traced by hand against the original C
  source and collapsed to plain `for` loops (`statMaxContextOTL` had 4
  nested occurrences). `unconsolidate_chaining` (expands a lookup's
  multi-rule "poly" chaining representation into one canonical subtable per
  rule — real ownership-transferring memory management, ~90 lines) had a
  first pass computing a `totalRules` count that's **never read afterward,
  confirmed dead in the original C too** (`c/lib/otf-reader/unconsolidate.c`);
  removed. Every other bounded loop across the four files became `for`;
  linked-list-style hash-table traversals (`while !p.is_null() { ...; p =
  next }`) were left as `while`, consistent with the rest of this pass.
  Deliberately skipped: `statOS_2UnicodeRanges` (~450 lines, but not a loop
  — one `if unicode-in-range { set bit N }` per OpenType-spec Unicode block,
  already flat; retyping ~150 bit positions by hand is high risk for a
  change that's mostly whitespace). Caught one real mistake mid-rewrite:
  `onCurve` is `int8_t`, not `bool` — a naive `onCurve as uint8_t` would NOT
  reproduce the original's "normalize any nonzero value to exactly 1"
  behavior; fixed to `(onCurve != 0) as uint8_t` before it was ever
  committed. Verified real coverage before touching
  `unconsolidate_chaining`: dumped all 6 TTF payloads and confirmed
  NotoNastaliqUrdu-Regular/iosevka-r/BungeeColor-Regular_colr_Windows/
  Molengo-Regular all have `gsub_chaining` (and NotoNastaliqUrdu also
  `gpos_chaining`) lookups, so the standard byte-comparison suite actually
  exercises the rewritten function.

Every commit in this pass was verified against the full byte-comparison
matrix (`compare-with-c.sh`), all round-trip payloads
(`compare-roundtrips.js`), and — for the bk and unconsolidate_chaining
changes specifically — the issue #1 golden regression test, before moving
to the next file.

## Next steps (Phase 2 continued)

- **Public vtable → trait conversion**: `bk_CellType`'s *type* (still a
  plain `c_uint`, not a real Rust `enum`) and the struct-of-function-pointer
  "interfaces" (`otl_iCoverage`, `otfcc_iHandle`, etc.) were deliberately
  left alone in this pass. They're referenced via duplicated `extern "C"`
  declarations in dozens of other files (there are no `use` statements
  anywhere pre-idiomatization — every file redeclares everything it calls),
  so changing their *public* shape needs a coordinated, crate-wide pass, not
  a local one.
- **Crate-wide rollout of `alloc.rs`/a `binio.rs`**: extend the dedup done
  here to the remaining files with their own copy of the alloc helpers, and
  factor out the also-duplicated `read_8u/16u/24u/32u` family (used
  throughout `lib/table/**`, unused in support/bk/otf_reader/otf_writer).
- **Redundant same-width casts** (`x as c_int == y as c_int` where both
  sides are already the same type, ~130 occurrences crate-wide) and ternary-
  cast-to-bool chains: skipped in this pass because — unlike the `true_0`/
  `false_0` sweep — safely removing them requires confirming per-site that
  both operands really are the same original width/signedness (and, per the
  `onCurve` mistake caught above, that the source field really is `bool`);
  the compiler only catches an outright type error, not a silently-wrong
  comparison or normalization from an incorrectly-dropped cast.
- Continue module by module: `table/otl` next (the OTL builder/reader is the
  largest single cluster, 60 files / 105K lines, and directly touches the
  issue #1 code path from the C side), then `consolidate`/`json_reader`/
  `json_writer`, then `libcff`/`glyf`/`vf`, per the original plan's ordering
  — keeping the round-trip tests green throughout.
- Once Rust is trusted as the sole implementation, retire the C build by
  deleting `c/` (`quick.make`, `premake5.lua`, `lib/`, `src/`, `dep/`,
  `include/` all live there now, precisely so this is a single directory
  removal) and `compare-with-c.sh`.
