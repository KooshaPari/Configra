# ADR-002: Async-First Configuration Loading

**Status**: Accepted  
**Date**: 2026-04-04  
**Deciders**: Conft Core Team

---

## Context

Configuration loading in Node.js libraries has traditionally been synchronous, relying on `readFileSync` and `process.env` access. This approach creates several problems:

1. **Event loop blocking**: File system operations block the event loop, reducing throughput
2. **Remote sources**: Synchronous APIs cannot support HTTP-based or secret management sources
3. **Consistency**: Mixed sync/async APIs create confusion about which methods to use
4. **Testing**: Synchronous file operations require more complex mocking

We need to decide whether Conft should adopt async-first design or provide both sync and async APIs.

---

## Decision

Conft will use **async-first design** for all configuration loading operations.

### Rationale

| Concern | Sync Approach | Async Approach |
|---------|---------------|----------------|
| Event loop | Blocks | Non-blocking |
| Remote sources | Impossible | Native support |
| API consistency | Mixed | Uniform |
| Testing | Complex mocking | Simple Promise mocking |
| Future-proofing | Limited | Extensible |

### Implementation

All `ConfigSource` implementations return `Promise`:

```typescript
export interface ConfigSource {
  readonly name: string;
  load(): Promise<ConfigEntry[]>;
  get(key: string): Promise<ConfigValue | undefined>;
  set(key: string, value: ConfigValue): Promise<void>;
}
```

The `ConfigService` follows the same pattern:

```typescript
export interface ConfigService<T> {
  load(): Promise<ConfigResult<T>>;
  reload(): Promise<ConfigResult<T>>;
  get(): T;  // Synchronous access after load
}
```

### Usage Pattern

```typescript
// Entry point: async initialization
async function main() {
  const configService = new ConfigService({
    schema: AppConfigSchema,
    sources: [
      new FileAdapter('./config/default.json'),
      new EnvAdapter({ prefix: 'APP_' }),
    ],
  });
  
  const result = await configService.load();
  if (!result.success) {
    console.error('Configuration failed:', result.errors);
    process.exit(1);
  }
  
  // After load: synchronous access is fine
  const config = configService.get();
  startServer(config.port);
}

main().catch(console.error);
```

---

## Consequences

### Positive

- **Non-blocking**: Event loop remains free for other operations
- **Remote ready**: Future adapters can fetch from HTTP, Vault, etc.
- **Consistent API**: All loading operations are async
- **Testability**: Easy to mock with `Promise.resolve()`
- **Modern patterns**: Aligns with async/await best practices

### Negative

- **Entry point change**: Top-level `await` or async wrapper required
- **Learning curve**: Developers must understand async initialization
- **Migration cost**: Existing sync-based code needs updates

### Mitigations

- Provide clear initialization patterns in documentation
- Include error handling examples for failed loads
- Document synchronous access pattern after initialization

---

## Alternatives Considered

### Alternative 1: Sync-Only API

**Rejected**: Prevents remote sources; blocks event loop; not future-proof.

### Alternative 2: Dual Sync/Async API

**Rejected**: Doubles API surface; encourages wrong choice (convenience over correctness); creates maintenance burden.

Example of the confusion dual APIs create:
```typescript
// Which should users choose?
config.loadSync();  // Blocking, convenient, wrong
await config.load();  // Non-blocking, correct
```

### Alternative 3: Lazy Loading with Caching

**Partially adopted internally**: First access triggers async load; subsequent accesses use cached value. However, explicit `load()` remains required for clarity and error handling.

---

## Related Decisions

- ADR-001: Zod for Schema Validation
- ADR-003: Deep Merge for Layered Config

---

## References

- [Node.js Event Loop](https://nodejs.org/en/docs/guides/event-loop-timers-and-nexttick/)
- [MDN: async function](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function)
