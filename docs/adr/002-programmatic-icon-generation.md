# ADR-002: Programmatic icon generation over static PNG assets

**Status:** Accepted
**Date:** 2026-03-02

## Context

The app needs a menu bar icon. Options considered:

1. **Embed a static PNG** — requires the `image` crate for decoding, plus a designer to produce 22x22 and 44x44 pixel assets.
2. **Generate the icon programmatically** — draw the bullseye (from the ITA logo) using distance-from-centre math directly into an RGBA buffer.

## Decision

We chose **programmatic generation**.

## Rationale

- **The bullseye is trivial geometry** — two concentric circles (a dot and a ring). This is ~30 lines of pixel math with anti-aliasing.
- **No additional dependencies** — avoids pulling in the `image` crate just to decode a PNG.
- **Monochrome by design** — black pixels with alpha transparency make it a macOS "template image" that automatically adapts to light/dark mode.
- **Scalable** — changing the icon size is a single parameter change; no new assets needed.
- **No external assets to manage** — everything is in the source code.

## Consequences

- The icon is a geometric approximation of the ITA bullseye, not a pixel-perfect brand asset.
- For a polished release, proper designer-produced PNG assets (22x22 @1x, 44x44 @2x) should replace the generated icon. The `icon.rs` module is structured to make this swap straightforward.
