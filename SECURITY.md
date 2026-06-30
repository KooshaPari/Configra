# Security Policy

## Supported versions

Configra is pre-1.0 and in active development. Security fixes are applied to
the latest `main` only.

| Version | Supported |
|---|---|
| 0.4.x (latest `main`) | Yes |
| < 0.4 | No |

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report via GitHub private security advisory:
<https://github.com/KooshaPari/Configra/security/advisories/new>

Please include:

- Crate(s) affected
- Description of the vulnerability and impact
- Steps to reproduce (or a minimal proof-of-concept)
- Suggested fix if you have one

You should receive acknowledgement within 72 hours. We aim to publish a fix and
advisory within 14 days for critical issues.

## Supply-chain policy

- Dependency advisories are checked weekly via `cargo-audit` (see
  `.github/workflows/deny.yml`)
- Justified RUSTSEC suppressions are documented inline in `deny.toml`
- A CycloneDX SBOM is generated on every CI run (`sbom.yml`)
- Secrets are scanned on every push via TruffleHog (`trufflehog.yml`)

## Cryptography note

`settly` uses AES-256-GCM with Argon2id key derivation for encryption-at-rest
(`src/crypto.rs`). This is gated behind the `encryption` Cargo feature flag and
is **not** enabled by default. The implementation follows OWASP 2024 Argon2id
parameter recommendations (m=64 MiB, t=3, p=4).
