# Functional Requirements — `@phenotype/config-ts`

These requirements describe the public contract shipped by version 0.1.x.
`traceability.json` maps every active requirement to executable tests and
implementation files; `bun run traceability` rejects missing or stale mappings.

| ID | Active requirement |
|----|--------------------|
| FR-SCHEMA-001 | The package SHALL export Zod schemas that accept every supported `ConfigValue` shape and reject unsupported shapes. |
| FR-SCHEMA-002 | `ConfigValidationError` SHALL retain the failing path, value, optional context, and a serializable error representation. |
| FR-SRC-001 | `FileConfigSource` SHALL load top-level entries from a JSON file and identify them as file-sourced. |
| FR-SRC-002 | `FileConfigSource.get()` SHALL return a named value or `undefined`, and the source SHALL report that it is read-only. |
| FR-SRC-003 | `EnvConfigSource` SHALL load only variables with its configured prefix and normalize loaded keys to lowercase. |
| FR-SRC-004 | `EnvConfigSource.get()` SHALL parse booleans, finite numbers, arrays, and string records while preserving ordinary strings. |
| FR-SRC-005 | Environment sources SHALL be read-only and return `undefined` for absent values. |
| FR-MODEL-001 | `ImmutableConfig` SHALL expose lookup, membership, and detached serializable snapshot operations. |
| FR-HEX-001 | The package SHALL expose the asynchronous `ConfigSource` port implemented by both source adapters. |
| FR-PKG-001 | The built ESM and CommonJS entry points SHALL expose the documented runtime API to downstream consumers. |

## Explicitly deferred

YAML/TOML adapters, writable file updates, source composition, deep-merge
layering, and secret-provider integration remain roadmap items. They are not
part of the 0.1.x contract and must not be presented as shipped behavior.
