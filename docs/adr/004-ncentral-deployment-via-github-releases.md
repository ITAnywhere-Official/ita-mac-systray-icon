# ADR-004: N-central deployment via GitHub Releases

**Status:** Accepted
**Date:** 2026-03-02

## Context

Client machines have N-central agents installed. We need a way to distribute the tray icon binary to macOS endpoints.

## Decision

We distribute via **GitHub Releases** and deploy using **N-central automation policies**.

## Rationale

- **Clients already have N-central agents** — no new infrastructure needed.
- **GitHub Releases provide a stable download URL** — `https://github.com/ITAnywhere-Official/ita-mac-systray-icon/releases/latest/download/ita-mac-systray-icon-<target>`.
- **No code signing required** — software deployed via a management agent (N-central) bypasses macOS Gatekeeper, as the agent is already trusted on the endpoint.
- **Simple update path** — push a new tag, CI builds new binaries, N-central scripts pull the latest.

## Consequences

- The repo is public, so binaries are accessible to anyone (acceptable for a URL launcher with no sensitive logic).
- N-central technicians need to configure automation policies for deployment and auto-start (`launchd` plist).
- If the repo were ever made private, the deployment script would need authentication (GitHub token).
