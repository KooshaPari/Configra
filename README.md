# phenotype-config

Local-first configuration, feature flags, secrets, and version tracking for Phenotype projects.

## Install

```bash
cargo install --path pheno-cli
```

## Quick Start

```bash
phenoctl config set app.name "My App"
phenoctl flags create dark-mode --description "Enable dark mode"
phenoctl flags enable dark-mode
phenoctl secrets set API_KEY
phenoctl version show
phenoctl tui
```
