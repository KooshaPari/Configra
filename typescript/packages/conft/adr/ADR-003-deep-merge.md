# ADR-003: Deep Merge Semantics for Layered Configuration

**Status**: Accepted  
**Date**: 2026-04-04  
**Deciders**: Conft Core Team

---

## Context

Configuration layering allows multiple sources to contribute to the final configuration. A common pattern is:

1. Default configuration (base.json)
2. Environment-specific configuration (production.json)
3. Environment variable overrides
4. Command-line argument overrides

The question is: how should these layers be combined when they contain nested objects?

Consider this scenario:

```json
// default.json
{
  "database": {
    "host": "localhost",
    "port": 5432,
    "pool": {
      "min": 2,
      "max": 10
    }
  }
}

// production.json override
{
  "database": {
    "host": "prod-db.example.com"
  }
}
```

**Shallow merge result** (loses nested defaults):
```json
{
  "database": {
    "host": "prod-db.example.com"
    // port, pool are LOST!
  }
}
```

**Deep merge result** (preserves nested defaults):
```json
{
  "database": {
    "host": "prod-db.example.com",
    "port": 5432,
    "pool": {
      "min": 2,
      "max": 10
    }
  }
}
```

We must decide which merge strategy Conft uses.

---

## Decision

Conft will use **deep merge** semantics for combining configuration layers.

### Behavior Specification

| Scenario | Behavior |
|----------|----------|
| Primitive override | New value replaces old value |
| Object merge | Properties are recursively merged |
| Array handling | Arrays are replaced, not concatenated |
| Null value | Null overrides existing value (not skipped) |
| Undefined value | Undefined property is ignored (not set) |

### Example

```typescript
const base = {
  server: {
    host: 'localhost',
    port: 3000,
    ssl: {
      enabled: false,
      cert: null,
    },
  },
  features: ['auth', 'logging'],
};

const override = {
  server: {
    host: '0.0.0.0',
    ssl: {
      enabled: true,
      key: '/path/to/key',
    },
  },
  features: ['auth', 'metrics'],  // Replaces, not merges
};

// Deep merge result
const merged = {
  server: {
    host: '0.0.0.0',
    port: 3000,  // Preserved from base
    ssl: {
      enabled: true,  // Overridden
      cert: null,     // Preserved from base
      key: '/path/to/key',  // Added from override
    },
  },
  features: ['auth', 'metrics'],  // Override wins
};
```

---

## Consequences

### Positive

- **Minimal overrides**: Override files only specify what differs
- **DRY principle**: Base configuration provides defaults; environments customize
- **Expected behavior**: Matches user intuition about "overriding"
- **Safe evolution**: Adding new defaults doesn't require updating all overrides

### Negative

- **Array behavior**: Arrays being replaced (not merged) may surprise some users
- **Complexity**: Deep merge is more complex than shallow merge
- **Performance**: Slightly more overhead than shallow merge
- **Ambiguity**: "Deep" can be interpreted differently (we follow lodash-style)

### Mitigations

- Array replacement behavior is documented prominently
- Future: `mergeStrategy: 'concat'` option for array merging if needed
- Maximum depth protection to prevent prototype pollution attacks

---

## Implementation Details

### Algorithm

```typescript
function deepMerge<T>(target: T, source: unknown): T {
  if (isPrimitive(source)) return source as T;
  if (Array.isArray(source)) return source as T;  // Replace arrays
  if (typeof source !== 'object' || source === null) return target;
  
  const result = { ...target } as Record<string, unknown>;
  
  for (const [key, value] of Object.entries(source)) {
    if (key === '__proto__' || key === 'constructor') continue;  // Security
    
    if (isPlainObject(value) && isPlainObject(result[key])) {
      result[key] = deepMerge(result[key], value);
    } else {
      result[key] = value;
    }
  }
  
  return result as T;
}
```

### Security Considerations

Deep merge implementations are vulnerable to prototype pollution if not careful:

```typescript
// Attack payload
const malicious = JSON.parse('{"__proto__": {"isAdmin": true}}');

// Safe implementation rejects __proto__ and constructor
```

Conft's merge:
- Explicitly rejects `__proto__` and `constructor` keys
- Operates on own properties only (`Object.entries`)
- Does not use `Object.prototype` methods that could be polluted

---

## Alternatives Considered

### Alternative 1: Shallow Merge Only

**Rejected**: Requires complete duplication of nested config in every override file. Violates DRY.

### Alternative 2: Arrays Concatenated

**Rejected**: Concatenation leads to unpredictable behavior with feature flags and similar arrays. Replacement is explicit and predictable.

Example of concatenation problems:
```typescript
// With concat: features accumulate unexpectedly
base: { features: ['auth'] }
env1: { features: ['logging'] }
env2: { features: ['metrics'] }
// Result with concat: ['auth', 'logging', 'metrics']
// Result with replace (Conft): ['metrics']
```

### Alternative 3: JSON Merge Patch (RFC 7386)

**Considered**: JSON Merge Patch uses `null` to indicate deletion and has specific rules.

**Not adopted**: Too complex for typical use cases; less intuitive than simple deep merge.

---

## Related Decisions

- ADR-001: Zod for Schema Validation
- ADR-002: Async-First Configuration Loading
- ADR-004: CompositeAdapter for Multi-Source Configuration

---

## References

- [lodash.merge](https://lodash.com/docs/4.17.15#merge)
- [deepmerge](https://github.com/TehShrike/deepmerge) npm package
- [Prototype Pollution](https://github.com/HoLyVieR/prototype-pollution-nsec18) security research
