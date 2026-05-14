# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

A benchmark comparing implementations of "find the first position in a string where the previous N characters are all unique" across JavaScript, Rust, and Zig. The canonical input is `input.txt` (4095 random characters) and the default window size is 14. Published as a blog post on posidevely.com.

## Running Implementations

**JavaScript** (Node v22.15.1 per `.nvmrc`):
```bash
node --allow-natives-syntax js/chris.js
node --allow-natives-syntax js/mike.js
```
`--allow-natives-syntax` is required — the JS files use V8 intrinsics (`%PrepareFunctionForOptimization`, `%OptimizeFunctionOnNextCall`) to ensure JIT-compiled benchmarking.

**Rust** (no Cargo, single-file compilation):
```bash
cd rust && rustc chris.rs && ./chris
cd rust && rustc benny.rs && ./benny
cd rust && rustc david.rs && ./david
```

**Zig**:
```bash
zig run zig/chris.zig
```

**Combinations utility** (standalone math script, unrelated to the main benchmark):
```bash
node combinations.js
```

## Architecture

All implementations solve the same problem with the same bitwise approach (except `js/mike.js` which uses a `Set` as a baseline):

- Read `input.txt` from the filesystem
- Use a bitmask/state variable where each bit represents whether a character (mapped via `& 31` or `% 32`) has been seen in the current window
- On duplicate, walk backward to clear bits until the duplicate is evicted
- Report the position and timing (usually 100-iteration average in nanoseconds)

The `& 31` trick (used in place of `% 26` or `% 32`) is intentional — it's the optimization being studied. The `godbolt/` directory contains stripped-down versions for https://godbolt.org to inspect generated assembly.

## Project Conventions

- No package manager, no build system, no linter — each file is self-contained
- Compiled Rust binaries (`rust/benny`, `rust/chris`, `rust/david`) are gitignored
- `WORKING.md` is gitignored (personal dev notes)
- Files are named by author (`chris.js`, `mike.js`, `benny.rs`, `david.rs`) to track who wrote which variant
