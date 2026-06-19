# Conft Charter

## Mission

Conft provides type-safe, validated configuration management for TypeScript applications with zero-configuration defaults, hexagonal architecture compliance, and production-grade reliability.

---

## Tenets (unless you know better ones)

These tenets guide Conft's development:

### 1. Type Safety First

All configuration must be fully typed. Runtime validation must produce TypeScript-compatible types. No `any` types in public APIs. The type system is not optional—it is the primary interface.

### 2. Fail Fast, Fail Loud

Invalid configuration must be caught at application startup, not at runtime. Error messages must identify the exact failing field, the expected type, and the actual value received. Silent failures are bugs.

### 3. Configuration as Code

Configuration schemas are code, not comments. They live in version control, are reviewed in pull requests, and are tested in CI. No external configuration systems that bypass the type checker.

### 4. Layered Sources

Configuration comes from multiple sources with predictable priority: defaults < files < environment < command line. Each layer can override the previous. Complex applications need this flexibility.

### 5. Hexagonal by Design

The domain knows nothing about files, environment variables, or network calls. Adapters handle external concerns. The domain depends only on ports (interfaces). This enables testing without filesystem mocks.

### 6. Zero Configuration Defaults

Sensible defaults must work out of the box. A new user should be productive in minutes, not hours. Convention over configuration, but configuration when needed.

### 7. Performance at Scale

Configuration loading must not block the event loop. File watching must use efficient OS primitives. Memory usage must be proportional to config size, not application size.

### 8. Observable Behavior

All configuration loading, reloading, and validation must be observable via events and hooks. Debugging configuration issues must not require source code modification.

---

## Charter Compliance Checklist

| Tenet | Verification Method |
|-------|---------------------|
| Type Safety First | `tsc --strict` passes; no `any` in exported types |
| Fail Fast, Fail Loud | Invalid config throws at `load()`; never returns partial success |
| Configuration as Code | All schemas in `.ts` files; version controlled |
| Layered Sources | `CompositeAdapter` with priority ordering |
| Hexagonal by Design | Domain has no `fs`, `process.env` imports |
| Zero Configuration Defaults | `new ConfigService()` works without arguments |
| Performance at Scale | Async loading; non-blocking file watching |
| Observable Behavior | `onChange`, `onError`, `onLoad` event hooks |

---

## Project Roles

### Core Maintainer

- Architecture decisions
- Release management
- Charter compliance

### Contributor

- Bug fixes
- Documentation improvements
- Adapter implementations

### User

- Application developers using Conft
- Infrastructure engineers defining deployment configs

---

## License

MIT - See LICENSE file for details.
