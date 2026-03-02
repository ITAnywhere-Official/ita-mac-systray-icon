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

```
src/
├── main.rs    — entry point, wires modules together, owns PORTAL_URL constant
├── icon.rs    — bullseye icon generation (pixel math, no external assets)
└── tray.rs    — tray icon construction, menu building, event loop
docs/
└── adr/       — architecture decision records
```

- `icon.rs` generates a monochrome bullseye icon programmatically (no image crate needed)
- `tray.rs` owns the menu bar icon setup and the blocking event loop
- `main.rs` is a thin entry point that passes the icon and portal URL to the tray module

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
- The bullseye icon is generated in code — see ADR-002 for rationale
- Architecture decisions are recorded in `docs/adr/`
