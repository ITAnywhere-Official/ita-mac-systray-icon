# CLAUDE.md — ita-mac-systray-icon

## Project Overview

A minimal macOS menu bar (system tray) application that provides a single link to the IT Anywhere self-service portal. Built in Rust.

## Build Commands

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run locally (macOS only — will not render a tray icon on Windows/Linux)
cargo run

# Check without building
cargo check

# Run clippy lints
cargo clippy

# Format code
cargo fmt
```

## Architecture

This is intentionally a tiny, single-file application:

- `src/main.rs` — the entire application
- Creates a menu bar icon using `tray-icon` crate
- Uses `tao` for the event loop
- Uses `open` to launch the browser

There are no modules, no config files, no tests (there is nothing to test — it's pure glue code).

## Key Constants

- `PORTAL_URL` in `main.rs` — the self-service portal URL (placeholder, pending confirmation)

## Platform

- **Target:** macOS only (will compile on other platforms but the tray icon won't behave correctly)
- **Toolchain:** Stable Rust
- **Dependencies:** Added via `cargo add` — never edit `Cargo.toml` `[dependencies]` directly

## Conventions

- Keep it minimal — this app does one thing and should stay simple
- No unnecessary abstractions or over-engineering
- British English in comments and documentation
- The icon is a placeholder — will be replaced with a branded asset
