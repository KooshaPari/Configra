# Comparison Matrix

## Feature Comparison

This document compares **phenotype-config** with similar tools in the configuration management, feature flags, and secrets management space.

| Repository | Purpose | Key Features | Language/Framework | Maturity | Comparison |
|------------|---------|--------------|-------------------|----------|------------|
| **phenotype-config (this repo)** | Configuration management | Feature flags, Secrets, Version tracking, TUI | Rust | Stable | Phenotype config management |
| [Unleash](https://github.com/Unleash/unleash) | Feature flags | Toggle management, SDKs, Analytics | Node.js | Stable | Enterprise feature flags |
| [Flagsmith](https://github.com/Flagsmith/flagsmith) | Feature flags | SaaS/Self-hosted, Segments | Python | Stable | Feature management |
| [Doppler](https://github.com/DopplerHQ/cli) | Secrets management | Env sync, Teams, Audit | Go | Stable | Secrets at scale |
| [Vault](https://github.com/hashicorp/vault) | Secrets management | Dynamic secrets, PKI, Encryption | Go | Stable | Enterprise secrets |
| [1Password CLI](https://github.com/1password/onepassword-operator) | Secrets management | 1Password integration, K8s | Go | Stable | Personal secrets |
| [fig.io](https://github.com/withfig/fig) | Config management | Autocomplete, Specs | TypeScript | Stable | CLI autocomplete |

## Detailed Feature Comparison

### Configuration Management

| Feature | phenotype-config | Unleash | Flagsmith | Doppler | Vault |
|---------|-----------------|---------|----------|---------|-------|
| App/Runtime Config | ✅ | ❌ | ❌ | ✅ | ✅ |
| Feature Flags | ✅ | ✅ | ✅ | ❌ | ❌ |
| Secret Storage | ✅ | ❌ | ❌ | ✅ | ✅ |
| Version Tracking | ✅ | ✅ | ✅ | ✅ | ✅ |
| Rollout State | ✅ | ✅ | ✅ | ❌ | ❌ |
| CLI-first | ✅ | ❌ | ❌ | ✅ | ✅ |

### Feature Flag Lifecycle

| Feature | phenotype-config | Unleash | Flagsmith |
|---------|-----------------|---------|----------|
| Create Flags | ✅ | ✅ | ✅ |
| Enable/Disable | ✅ | ✅ | ✅ |
| Flag Descriptions | ✅ | ✅ | ✅ |
| Toggle History | ❌ | ✅ | ✅ |
| Gradual Rollouts | ❌ | ✅ | ✅ |

### CLI/TUI

| Feature | phenotype-config | Doppler | Vault |
|---------|-----------------|---------|-------|
| CLI Tool | ✅ (phenoctl) | ✅ | ✅ |
| TUI | ✅ | ❌ | ❌ (Web UI) |
| Interactive Workflows | ✅ | ❌ | ❌ |

## Unique Value Proposition

phenotype-config provides:

1. **Local-First**: Designed for local development and team workflows
2. **CLI-First**: Terminal UI for operational workflows
3. **Feature Flags + Config**: Combined config, flags, and secrets
4. **Phenotype Integration**: Part of Phenotype ecosystem

## Commands

```bash
phenoctl config set app.name "My App"   # Set config
phenoctl flags create dark-mode          # Create flag
phenoctl flags enable dark-mode         # Enable flag
phenoctl secrets set API_KEY            # Set secret
phenoctl version show                   # Show version
phenoctl tui                           # Terminal UI
```

## Repository Structure

```
phenotype-config/
├── pheno-cli/        # CLI implementation (Rust)
├── docs/             # VitePress documentation
└── config/           # Default configuration
```

## When to Use What

| Use Case | Recommended Tool |
|----------|-----------------|
| Phenotype ecosystem | phenotype-config |
| Enterprise feature flags | Unleash, Flagsmith |
| Secrets at scale | Doppler, Vault |
| Local secrets | 1Password CLI |
| CLI autocomplete | fig.io |

## References

- Unleash: [Unleash/unleash](https://github.com/Unleash/unleash)
- Flagsmith: [Flagsmith/flagsmith](https://github.com/Flagsmith/flagsmith)
- Doppler: [DopplerHQ/cli](https://github.com/DopplerHQ/cli)
- Vault: [hashicorp/vault](https://github.com/hashicorp/vault)
