# Functional Requirements -- phenotype-config

## FR-CFG: Configuration Management

### FR-CFG-001: Config Set
`phenoctl config set <key> <value>` SHALL persist the entry to SQLite with type metadata.
**Traces to:** E1.1

### FR-CFG-002: Config Get
`phenoctl config get <key>` SHALL retrieve the entry with its current value and type.
**Traces to:** E1.1

### FR-CFG-003: Audit Trail
Every config mutation SHALL produce an audit record with timestamp, actor, old value, and new value.
**Traces to:** E1.2

### FR-CFG-004: Point-in-Time Restore
`phenoctl config restore --at <timestamp>` SHALL restore config state to the specified point in time.
**Traces to:** E1.2

## FR-FLG: Feature Flags

### FR-FLG-001: Flag Create
`phenoctl flags create <name> --description <desc>` SHALL create a feature flag in the disabled state.
**Traces to:** E2.1

### FR-FLG-002: Flag Toggle
`phenoctl flags enable/disable <name>` SHALL toggle the flag state and record an audit entry.
**Traces to:** E2.1

## FR-SEC: Secret Storage

### FR-SEC-001: Secret Set
`phenoctl secrets set <key>` SHALL prompt for the value without echo and store it encrypted via AES-256-GCM.
**Traces to:** E3.1

### FR-SEC-002: Secret Get
`phenoctl secrets get <key>` SHALL decrypt and display the secret value.
**Traces to:** E3.1

## FR-VER: Version Tracking

### FR-VER-001: Version Show
`phenoctl version show` SHALL display current version, build metadata, and rollout state.
**Traces to:** E4.1

## FR-TUI: Terminal UI

### FR-TUI-001: Interactive TUI
`phenoctl tui` SHALL launch a ratatui-based interface with panels for config, flags, secrets, and version management.
**Traces to:** E5.1
