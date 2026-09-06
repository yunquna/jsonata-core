# YQN WASM fork

This fork retains the upstream MIT license and high-level Rust Expression API.
Upstream: https://github.com/txjmb/jsonata-core
Base: da7e15fa257bb048d053901c6b5c0441d88ca187 (2.2.9 manifest).

## Current patch

- Enable getrandom's JavaScript backend only on wasm32.
- Use web-time for wasm32 evaluator/VM clocks; keep std::time on native targets.
- Keep native stacker calls unchanged; wasm32 uses a fixed stack with parser,
  transform and evaluator ceilings of 64 instead of native stack growth.
- Optional `regex-lookaround` tries the existing linear regex engine first and
  falls back to fancy-regex with a 1,000,000 backtracking limit. Execution errors
  propagate through contains, split, replace, match and regex invocation.
- Regex result limits apply before matching, and an empty `$match` returns
  Undefined (including limit=0), matching the reference semantics.
- Python, SIMD, CLI and allocator features are not required for the WASM build.

Use `default-features = false` for wasm32. Pin a tested Git revision; the upstream
manifest version is not a claim that this fork was published to crates.io.

## Verification boundary

Native: existing library tests pass with and without regex-lookaround (188 each).
Regex regression tests cover default rejection (2 tests), and lookahead entry
points, backtracking and zero/small limits (4 tests with the feature).

A separate Components wrapper was built with wasm-pack and executed under
workerd 1.20260826.1 / compatibility date 2026-09-02. All 16 deterministic
qualification cases matched jsonata-js 2.2.2 (including invalid-syntax rejection;
error messages/codes are not asserted byte-identical). This is a representative
corpus, not full JSONata conformance or Cloudflare production qualification.

Evaluation timeout and range/depth rejection were observed for finite test
cases. These checkpoints are not a universal hard CPU deadline: callers must
bound input/expression/output size and retain host CPU/memory containment.
Upstream's sequence guard does not cover every literal-array construction.
The current API also does not register `$random`; do not imply that every
JSONata builtin is covered by this patch.

Components owns the Worker/RPC wrapper and qualification corpus. This repository
owns the reusable Rust engine patch; it does not contain business models,
credentials, Artifact storage or Workflow orchestration.

## Following upstream

Fetch upstream and inspect its change against the recorded base. Rebase/merge
only on a working branch, preserve this small target-specific patch, then run
native, wasm-build and the Components workerd corpus before advancing a pinned
consumer revision. Do not auto-update production from upstream main.

The YQN workflow checks this Rust/WASM slice. Upstream Python packaging/release
and docs-deployment workflows are not release instructions for YQN; no registry
publish is performed by this patch.
