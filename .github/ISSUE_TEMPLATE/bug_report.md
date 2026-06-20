---
name: Bug report
description: Report a defect, crash, or unexpected behavior in Configra.
title: "[bug]: "
labels: ["bug", "triage"]
assignees: ["@kooshapari"]
---

## Summary

<!-- A one-sentence description of the bug. -->

## Environment

| Field            | Value                                |
| ---------------- | ------------------------------------ |
| Configra version | <!-- e.g. `0.1.0` or `main @ <sha>` --> |
| Rust toolchain   | <!-- `rustc --version` output -->     |
| OS               | <!-- e.g. macOS 14.5 / Ubuntu 24.04 --> |
| Architecture     | <!-- e.g. `aarch64-apple-darwin` -->  |
| Install method   | <!-- source / `cargo install` / Conft --> |

## Reproduction

<!-- Minimal steps to reproduce the bug. -->

```bash
# shell session here
```

```rust
// Rust snippet here
```

## Expected behavior

<!-- What you expected to happen. -->

## Actual behavior

<!-- What actually happened. Include stack traces, panic messages, etc. -->

```
paste output here
```

## Possible cause

<!-- Optional: your guess at the root cause or relevant source location. -->

## Additional context

<!-- Anything else that may help — logs, screenshots, related issues. -->

## Checklist

- [ ] I have searched [existing issues](../../issues) for duplicates.
- [ ] I have run `just ci` locally and the failure reproduces.
- [ ] I am willing to send a PR (optional).