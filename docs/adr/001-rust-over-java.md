# ADR-001: Rust over Java for macOS tray icon

**Status:** Accepted
**Date:** 2026-03-02

## Context

We need a lightweight macOS menu bar application that opens a self-service portal URL. The two candidate languages were Java and Rust.

## Decision

We chose **Rust**.

## Rationale

- **Java's `SystemTray` is poor on macOS** — AWT-based tray icons look non-native, have rendering quirks, and show an unwanted Dock icon. Apple's Human Interface Guidelines discourage this approach.
- **Rust produces a single native binary** (~2–5 MB) with no runtime dependencies. Java would require a JRE on every target machine.
- **Minimal footprint** — the app does one thing (open a URL). A JVM is disproportionate overhead for this.
- **macOS-native behaviour** — the `tray-icon` and `tao` crates provide proper macOS integration.

## Consequences

- Developers need a Rust toolchain to build locally.
- Cross-compilation to macOS from Windows/Linux requires additional setup (solved via GitHub Actions CI).
