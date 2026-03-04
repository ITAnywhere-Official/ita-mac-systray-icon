# ita-mac-systray-icon

A lightweight macOS menu bar application that provides quick access to the IT Anywhere self-service portal.

## What It Does

- Sits in the macOS menu bar as a small icon
- Click to reveal a menu with **"Open Self-Service Portal"** — opens the portal in the default browser
- **"Quit"** to exit the application
- No other functionality; intentionally minimal

## Platform Notes

This is a **macOS-targeted** application, but the code is cross-platform (the `tray-icon`, `tao`, and `open` crates all support Windows, macOS, and Linux). It will compile and run on Windows for development and testing purposes.

Key differences when running on non-macOS:

- **Icon appearance** — the ITA logo icon is designed for macOS menu bars. On Windows it appears as a standard system tray icon (functional, but not macOS-native looking).
- **Deployment** — the `.app` bundle, `.dmg` installer, and `launchd` plist are macOS-specific. Windows is not a supported deployment target.

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- macOS target (this is a macOS-only application)

### Compile

```bash
cargo build --release
```

The binary will be at `target/release/ita-mac-systray-icon`.

### Packaging (.app / .dmg)

To produce a macOS `.app` bundle and `.dmg` installer:

```bash
cargo install cargo-packager --locked
cargo packager --release
```

Output is written to `target/release/` (e.g., `IT Anywhere Tray.app`, `IT Anywhere Tray.dmg`).

### Cross-Compiling from Non-Mac

If building on a CI runner or non-Mac machine, you'll need the macOS SDK and appropriate cross-compilation target:

```bash
rustup target add aarch64-apple-darwin   # Apple Silicon
rustup target add x86_64-apple-darwin    # Intel Mac
cargo build --release --target aarch64-apple-darwin
```

> **Note:** Cross-compiling for macOS from Linux/Windows requires additional toolchain setup (e.g., [osxcross](https://github.com/tpober/osxcross)). Building natively on a Mac or using macOS CI runners is recommended.

## Downloads

A `.dmg` installer is available from GitHub Releases:

**Latest release:** https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/latest

Download the `.dmg`, open it, and drag **IT Anywhere Tray** to Applications (or deploy via your RMM/MDM tool).

> **Note:** macOS may block the app as it's from an unidentified developer. Go to **System Settings → Privacy & Security** and click "Allow Anyway", then open it again.

## Deployment

This application is intended to be deployed to client machines via **N-central** (or any RMM/MDM tool). When deployed via a management agent, macOS Gatekeeper is bypassed and no code signing is required.

### Deployment Steps

1. Download the `.dmg` from [GitHub Releases](https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/latest)
2. Install the `.app` to the target Mac endpoint (e.g., `/Applications/IT Anywhere Tray.app`)
3. Install the launch agent plist so it starts automatically at user login

### Launch Agent (Auto-Start)

Create the file `~/Library/LaunchAgents/com.itanywhere.systray.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.itanywhere.systray</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Applications/IT Anywhere Tray.app/Contents/MacOS/ita-mac-systray-icon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>
```

Then load it:

```bash
launchctl load ~/Library/LaunchAgents/com.itanywhere.systray.plist
```

### N-central Deployment Script

An N-central automation policy can run a script like:

```bash
#!/bin/bash
# Deploy IT Anywhere Tray via DMG

DMG_URL="https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/latest/download/IT-Anywhere-Tray.dmg"
DMG_PATH="/tmp/IT-Anywhere-Tray.dmg"
APP_NAME="IT Anywhere Tray.app"
PLIST_PATH="/Library/LaunchAgents/com.itanywhere.systray.plist"

# Download and mount DMG
curl -fsSL "$DMG_URL" -o "$DMG_PATH"
hdiutil attach "$DMG_PATH" -nobrowse -quiet
cp -R "/Volumes/IT Anywhere Tray/$APP_NAME" "/Applications/"
hdiutil detach "/Volumes/IT Anywhere Tray" -quiet
rm "$DMG_PATH"

# Install launch agent (for all users)
cat > "$PLIST_PATH" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.itanywhere.systray</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Applications/IT Anywhere Tray.app/Contents/MacOS/ita-mac-systray-icon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>
EOF
```

## Configuration

The self-service portal URL is currently compiled into the binary:

```
https://itanywhere.halopsa.com/portal/
```

> **TODO:** Confirm the final portal URL.

## Icon

The menu bar icon uses the official ITA logo. Two PNG sizes are provided:

- **22x22 px** (`icon-22x22.png`) — @1x for standard displays
- **44x44 px** (`icon-44x44.png`) — @2x for Retina displays

Both PNGs are generated from `assets/icon.svg`, a true vector SVG (paths, not embedded bitmaps). To regenerate after editing the SVG:

```bash
resvg assets/icon.svg assets/icon-22x22.png -w 22 -h 22
resvg assets/icon.svg assets/icon-44x44.png -w 44 -h 44
```

The 44x44 PNG is embedded in the binary at compile time via `include_bytes!`. Install `resvg` with `cargo install resvg`.

## Licence

Proprietary — Copyright (c) 2026 IT Anywhere. All rights reserved.
