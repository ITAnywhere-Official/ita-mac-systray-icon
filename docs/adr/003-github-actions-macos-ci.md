# ADR-003: GitHub Actions for macOS CI builds

**Status:** Accepted
**Date:** 2026-03-02

## Context

The development team works on Windows. macOS binaries cannot be built on Windows without significant cross-compilation toolchain setup (osxcross, macOS SDK).

## Decision

We use **GitHub Actions with `macos-latest` runners** to build release binaries, triggered by version tags (`v*`).

## Rationale

- **Free for public repositories** — unlimited CI minutes on GitHub's macOS runners.
- **Builds both architectures** — Apple Silicon (`aarch64-apple-darwin`) and Intel (`x86_64-apple-darwin`) via a build matrix.
- **Automated releases** — tag push triggers build, binaries are attached to a GitHub Release automatically.
- **Distribution channel** — technicians download binaries directly from the Releases page or script it via the predictable `releases/latest/download/` URL pattern.

## Consequences

- No local macOS build capability is needed.
- Release process is: `git tag vX.Y.Z && git push origin vX.Y.Z`.
- Debugging macOS-specific issues requires either a Mac or iterating through CI.
