# Security Policy — Configra
# SPDX-License-Identifier: MIT OR Apache-2.0

Configra takes the security of its code and its users seriously. This
document explains how to report vulnerabilities, what to expect from the
maintainers, and the supported versions.

---

## Supported Versions

| Version | Supported          | Notes                              |
| ------- | ------------------ | ---------------------------------- |
| `main`  | :white_check_mark: | Active development branch          |
| `0.1.x` | :white_check_mark: | Pre-1.0 — fixes for current minor  |
| `<0.1`  | :x:                | No longer maintained               |

Configra is currently pre-1.0; only the latest minor receives security
patches. Backports to older minors are at the maintainers' discretion.

## Reporting a Vulnerability

**Please do not file a public GitHub issue for suspected vulnerabilities.**

Instead, use one of the following private channels:

1. **GitHub private vulnerability disclosure** (preferred):
   <https://github.com/KooshaPari/Configra/security/advisories/new>
2. **Email**: `security@phenotype.org` (PGP key on request)
3. **GitHub private message**: `@kooshapari`

When reporting, please include:

- A clear description of the vulnerability and its impact.
- A minimal reproduction (Cargo snippet, shell commands, or test case).
- The commit/tag/branch where you observed the issue.
- Your name/handle for the advisory credits (if desired).

### What to expect

| Stage            | SLA             | Description                                              |
| ---------------- | --------------- | -------------------------------------------------------- |
| Acknowledgement  | **48 hours**    | We confirm receipt of your report.                       |
| Initial triage   | **5 business days** | We assess severity (CVSSv3.1) and reproduction.       |
| Patch cadence    | **30 days**     | Critical / high → within 30 days; medium → within 60.    |
| Public disclosure| After fix lands | We coordinate disclosure timing with you.               |

We follow [coordinated disclosure](https://en.wikipedia.org/wiki/Coordinated_vulnerability_disclosure):
please give us a reasonable window to investigate and patch before
publishing details.

## Threat Model (Scope)

In scope for Configra:

- **Secret confidentiality at rest** — AES-256-GCM ciphertext integrity,
  nonce reuse, key derivation.
- **Configuration tampering** — SQLite storage integrity, audit-trail
  immutability, point-in-time restore correctness.
- **CLI argument injection** — clap-derived parsers, file-path handling.
- **Dependency vulnerabilities** — see `deny.toml` and
  `.github/workflows/audit.yml`.
- **Supply chain** — SHA-pinned GitHub Actions, signed tags, reproducible
  builds where feasible.

Out of scope:

- Vulnerabilities in user-supplied TOML/JSON parsers when consumed via the
  public `Config::from_str` API without `schema = "strict"` mode.
- Issues in third-party crates that don't affect Configra's exposed surface.
- Social-engineering or infrastructure issues (e.g. compromised GitHub
  account) — report directly to GitHub.

## Security Tooling

Configra ships with the following automated defenses:

- `cargo-deny` — license, advisory, ban, source checks (`.github/workflows/deny.yml`).
- `cargo-audit` — RustSec advisory DB (`.github/workflows/audit.yml`).
- OpenSSF Scorecard — health score, weekly run (`.github/workflows/scorecard.yml`).
- SHA-pinned GitHub Actions in every workflow.
- Branch protection on `main` (required status checks, 1 review, no
  force-push).

## Cryptography

Configra uses **AES-256-GCM** for secret encryption at rest. Key material
is sourced from the operating system keychain (Linux: `keyutils`,
macOS: Keychain, Windows: DPAPI). We do **not** roll our own crypto.

If you find a flaw in the crypto layer, treat it as a critical-severity
issue and follow the reporting process above.

## Security Hall of Fame

We recognize reporters who follow coordinated disclosure. With your
permission, your handle will appear in the next release notes. Anonymous
reports are honored equally but won't be named.

---

_Last updated: 2026-06-20_