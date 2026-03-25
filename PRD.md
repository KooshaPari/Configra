# Product Requirements Document -- phenotype-config

## Product Vision

phenotype-config is a local-first configuration SDK for Phenotype projects, providing configuration management, feature flags, secret storage, and version tracking through a Rust workspace with a `phenoctl` CLI and TUI.

## E1: Configuration Management

### E1.1: Config CRUD

As a developer, I can set, get, list, and delete configuration entries via CLI.

**Acceptance Criteria:**
- `phenoctl config set <key> <value>` persists to SQLite
- `phenoctl config get <key>` retrieves with type information
- `phenoctl config list` shows all entries with metadata
- All changes produce audit trail entries

### E1.2: Audit Trail

As an operator, I can see the full history of configuration changes with who/what/when.

**Acceptance Criteria:**
- Every mutation records: timestamp, actor, old value, new value
- Point-in-time restore: `phenoctl config restore --at <timestamp>`

## E2: Feature Flags

### E2.1: Flag Lifecycle

As a developer, I can create, enable, disable, and delete feature flags.

**Acceptance Criteria:**
- `phenoctl flags create <name> --description <desc>` creates a flag
- `phenoctl flags enable/disable <name>` toggles the flag
- Flags queryable by name with current state and metadata

## E3: Secret Storage

### E3.1: Encrypted Secrets

As a developer, I can store secrets that are encrypted at rest using AES-256-GCM.

**Acceptance Criteria:**
- `phenoctl secrets set <key>` prompts for value (no CLI echo)
- Values encrypted via pheno-crypto before storage
- `phenoctl secrets get <key>` decrypts and displays

## E4: Version Tracking

### E4.1: Version Inspection

As a developer, I can inspect current version and rollout state.

**Acceptance Criteria:**
- `phenoctl version show` displays current version, build info, rollout state
- Version info sourced from build metadata and config store

## E5: Terminal UI

### E5.1: Interactive TUI

As an operator, I can manage config, flags, secrets, and versions through an interactive terminal UI.

**Acceptance Criteria:**
- `phenoctl tui` opens ratatui-based interface
- Navigate between config, flags, secrets, version panels
- Inline editing with confirmation prompts
