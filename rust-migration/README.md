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

## Status

The pipeline transpiles **all 118 translation units** to Rust in one pass with
c2rust 0.22.1 on clang-17, emitting a full Cargo crate (`Cargo.toml`, `lib.rs`,
`build.rs`, the three binaries, ~200k+ LOC of unsafe Rust).

> Non-obvious gotcha (handled by `transpile.sh`): the compile database **must**
> live on the mounted filesystem, not the container's `/tmp`. With the
> byte-identical DB on `/tmp`, c2rust panics reliably ("Type conversion not
> implemented for TagTypeUnknown"); on the bind mount it always succeeds.

## Next steps (Phase 1 completion)

- `cargo +nightly build` the transpiled project; fix whatever c2rust could not
  translate cleanly (unsupported syntax, macro-expanded vector helpers, link
  flags).
- Run the existing Node round-trip suite against the Rust binaries.
- Wire a CI job (cargo build + round-trip) alongside the C build.
