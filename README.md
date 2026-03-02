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

- **Template image tinting** — the monochrome black+alpha icon is designed for macOS's automatic light/dark mode tinting. On Windows it appears as a plain dark icon in the system tray (functional, but not macOS-native looking).
- **Deployment** — the `launchd` plist and N-central deployment scripts in this README are macOS-specific. Windows auto-start would use the registry or startup folder instead, but this is not a supported deployment target.

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- macOS target (this is a macOS-only application)

### Compile

```bash
cargo build --release
```

The binary will be at `target/release/ita-mac-systray-icon`.

### Cross-Compiling from Non-Mac

If building on a CI runner or non-Mac machine, you'll need the macOS SDK and appropriate cross-compilation target:

```bash
rustup target add aarch64-apple-darwin   # Apple Silicon
rustup target add x86_64-apple-darwin    # Intel Mac
cargo build --release --target aarch64-apple-darwin
```

> **Note:** Cross-compiling for macOS from Linux/Windows requires additional toolchain setup (e.g., [osxcross](https://github.com/tpober/osxcross)). Building natively on a Mac or using macOS CI runners is recommended.

## Downloads

Pre-built macOS binaries are available from GitHub Releases:

**Latest release:** https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/tag/v0.1.0

| Binary                                      | Target                      |
|---------------------------------------------|-----------------------------|
| `ita-mac-systray-icon-aarch64-apple-darwin` | Apple Silicon (M1/M2/M3/M4) |
| `ita-mac-systray-icon-x86_64-apple-darwin`  | Intel Mac                   |

### Quick Test

```bash
chmod +x ~/Downloads/ita-mac-systray-icon-*
~/Downloads/ita-mac-systray-icon-*
```

> **Note:** macOS may block the binary as it's from an unidentified developer. Go to **System Settings → Privacy & Security** and click "Allow Anyway", then run it again.

## Deployment

This application is intended to be deployed to client machines via **N-central** (or any RMM/MDM tool). When deployed via a management agent, macOS Gatekeeper is bypassed and no code signing is required.

### Deployment Steps

1. Download the appropriate binary from [GitHub Releases](https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/latest)
2. Distribute the binary to the target Mac endpoint (e.g., `/usr/local/bin/ita-mac-systray-icon`)
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
        <string>/usr/local/bin/ita-mac-systray-icon</string>
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
# Deploy ita-mac-systray-icon

BINARY_URL="https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/latest/download/ita-mac-systray-icon-aarch64-apple-darwin"
INSTALL_PATH="/usr/local/bin/ita-mac-systray-icon"
PLIST_PATH="/Library/LaunchAgents/com.itanywhere.systray.plist"

# Download binary
curl -fsSL "$BINARY_URL" -o "$INSTALL_PATH"
chmod +x "$INSTALL_PATH"

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
        <string>/usr/local/bin/ita-mac-systray-icon</string>
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

The application generates a **bullseye icon** programmatically — inspired by the target/bullseye motif in the IT Anywhere logo. It's drawn as monochrome black with alpha transparency, making it a macOS "template image" that automatically adapts to light and dark menu bars.

No external image files or additional dependencies are needed.

### Improving the Icon

For a polished release, the programmatic icon should be replaced with designer-produced PNG assets:

- **22x22 px** (@1x) for standard displays
- **44x44 px** (@2x) for Retina displays
- PNG format with transparency
- Monochrome recommended (macOS applies light/dark mode tinting automatically to template images)
- The bullseye/target from the ITA logo is the recommended motif

See [ADR-002](docs/adr/002-programmatic-icon-generation.md) for the rationale behind the current approach.

## Licence

Proprietary — Copyright (c) 2026 IT Anywhere. All rights reserved.
