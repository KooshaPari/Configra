# ADR-004: CompositeAdapter for Multi-Source Configuration

**Status**: Accepted  
**Date**: 2026-04-04  
**Deciders**: Conft Core Team

---

## Context

Real-world applications need configuration from multiple sources with predictable precedence:

1. Built-in defaults (lowest priority)
2. Configuration files (environment-specific)
3. Environment variables (deployment overrides)
4. Command-line arguments (highest priority)

The challenge is composing these sources while maintaining:
- Clear precedence rules
- Testability of individual sources
- Ability to add/remove sources without code changes
- Consistent error handling across sources

We need a design pattern that enables multi-source configuration without cluttering the domain layer.

---

## Decision

Implement a `CompositeAdapter` that implements the `ConfigSource` port while internally managing multiple child sources with priority ordering.

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     CompositeAdapter                              │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Implements ConfigSource                                  │   │
│  │  • load(): Aggregates all child sources                   │   │
│  │  • get(): Delegates to appropriate child                  │   │
│  │  • Composite of multiple adapters                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│         ┌────────────────────┼────────────────────┐             │
│         │                    │                    │             │
│         ▼                    ▼                    ▼             │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐        │
│  │   Memory     │   │    File      │   │     Env      │        │
│  │  Adapter     │   │   Adapter    │   │   Adapter    │        │
│  │ (priority 1) │   │ (priority 2) │   │ (priority 3) │        │
│  └──────────────┘   └──────────────┘   └──────────────┘        │
│       (defaults)       (config.json)      (process.env)         │
└─────────────────────────────────────────────────────────────────┘
```

### Interface

```typescript
export interface CompositeSourceOptions {
  sources: Array<{
    adapter: ConfigSource;
    priority: number;  // Higher number = higher priority
  }>;
  strategy: 'merge' | 'override';
}

export class CompositeAdapter implements ConfigSource {
  constructor(options: CompositeSourceOptions);
  
  async load(): Promise<ConfigEntry[]>;
  async get(key: string): Promise<ConfigValue | undefined>;
}
```

### Precedence Rules

Sources are sorted by priority (ascending). Lower priority sources are loaded first, then merged with higher priority sources:

```typescript
// Lower number = lower priority = loaded first
const sources = [
  { adapter: defaultsAdapter, priority: 10 },   // Base values
  { adapter: fileAdapter, priority: 20 },       // File overrides
  { adapter: envAdapter, priority: 30 },          // Env overrides
];

// Execution: defaults -> merge(file) -> merge(env)
// Result: env values win for conflicts
```

---

## Consequences

### Positive

- **Single interface**: Domain layer sees one `ConfigSource`, not multiple
- **Precedence clarity**: Priority numbers make ordering explicit
- **Flexibility**: Add/remove sources without changing domain code
- **Testability**: Test with different composite configurations
- **Gradual loading**: Sources can be loaded incrementally; failures isolated

### Negative

- **Complexity**: Additional abstraction layer
- **Debugging**: Multi-source resolution can be hard to trace
- **Error attribution**: Need clear error messages indicating which source failed

### Mitigations

- Each source is tagged with its name for debugging
- Errors include source attribution: `ConfigError { source: 'env', key: 'port' }`
- Optional `debug: true` mode logs merge decisions

---

## Implementation Details

### Loading Algorithm

```typescript
async function loadComposite(
  sources: Array<{ adapter: ConfigSource; priority: number }>
): Promise<ConfigEntry[]> {
  // Sort by priority ascending (lowest first)
  const sorted = sources.sort((a, b) => a.priority - b.priority);
  
  // Load all sources in parallel
  const results = await Promise.allSettled(
    sorted.map(({ adapter }) => adapter.load())
  );
  
  // Merge results from lowest to highest priority
  let merged: Record<string, ConfigValue> = {};
  const errors: ConfigError[] = [];
  
  for (let i = 0; i < sorted.length; i++) {
    const result = results[i];
    const { adapter, priority } = sorted[i];
    
    if (result.status === 'rejected') {
      errors.push({
        source: adapter.name,
        message: result.reason.message,
        priority,
      });
      continue;
    }
    
    // Convert entries to record for merging
    const record = Object.fromEntries(
      result.value.map(e => [e.key, e.value])
    );
    
    // Deep merge: current result overrides merged
    merged = deepMerge(merged, record);
  }
  
  if (errors.length > 0 && Object.keys(merged).length === 0) {
    throw new CompositeLoadError(errors);
  }
  
  return Object.entries(merged).map(([key, value]) => ({
    key,
    value,
    source: 'composite',
  }));
}
```

### Error Handling

```typescript
class CompositeLoadError extends Error {
  constructor(
    public readonly sourceErrors: Array<{
      source: string;
      message: string;
      priority: number;
    }>
  ) {
    super(
      `Failed to load configuration from all sources:\n` +
      sourceErrors.map(e => `  - ${e.source} (priority ${e.priority}): ${e.message}`).join('\n')
    );
  }
}
```

---

## Usage Examples

### Standard Web Application

```typescript
const composite = new CompositeAdapter({
  sources: [
    // Priority 10: Built-in defaults
    { adapter: new MemoryAdapter({
      port: 3000,
      host: 'localhost',
    }), priority: 10 },
    
    // Priority 20: Environment-specific file
    { adapter: new FileAdapter('./config/default.json'), priority: 20 },
    { adapter: new FileAdapter('./config/local.json'), priority: 25 },
    
    // Priority 30: Environment variables (highest)
    { adapter: new EnvAdapter({ prefix: 'APP_' }), priority: 30 },
  ],
  strategy: 'merge',
});

const configService = new ConfigService({
  schema: AppConfigSchema,
  source: composite,
});
```

### Testing with Isolated Sources

```typescript
// Unit test: Only memory adapter
const testService = new ConfigService({
  schema: TestSchema,
  source: new CompositeAdapter({
    sources: [
      { adapter: new MemoryAdapter({ testMode: true }), priority: 1 },
    ],
  }),
});
```

---

## Alternatives Considered

### Alternative 1: ConfigService Handles Multiple Sources

**Rejected**: Clutters the domain layer; violates single responsibility principle.

```typescript
// Don't do this: Service knows too much about sources
class ConfigService {
  constructor(options: {
    schema: ZodSchema<T>;
    sources: ConfigSource[];  // Service handles composition?
  }) {}
}
```

### Alternative 2: Chain of Responsibility Pattern

**Rejected**: Sequential loading is less efficient than parallel; error handling more complex.

### Alternative 3: User-Land Composition

**Rejected**: Forces users to implement merging logic; inconsistent across projects.

```typescript
// Don't force this on users:
const fileConfig = await fileAdapter.load();
const envConfig = await envAdapter.load();
const merged = deepMerge(fileConfig, envConfig);
```

---

## Related Decisions

- ADR-001: Zod for Schema Validation
- ADR-002: Async-First Configuration Loading
- ADR-003: Deep Merge Semantics for Layered Configuration

---

## References

- [Composite Pattern](https://refactoring.guru/design-patterns/composite) - Design Patterns
- [Chain of Responsibility](https://refactoring.guru/design-patterns/chain-of-responsibility) - Alternative considered
- [Priority Queue](https://en.wikipedia.org/wiki/Priority_queue) - Precedence mechanism
