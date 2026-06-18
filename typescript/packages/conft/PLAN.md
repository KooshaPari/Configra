# Conft — PLAN.md

## Implementation Plan

### Phase 1: Core (Week 1)

| Task | Description | Deliverable |
|------|-------------|-------------|
| P1.1 | Project setup | TypeScript, tsup, Vitest |
| P1.2 | Domain models | Config, ConfigError types |
| P1.3 | ConfigSource port | Interface definition |
| P1.4 | FileAdapter | JSON/YAML loading |

### Phase 2: Adapters (Week 2)

| Task | Description | Deliverable |
|------|-------------|-------------|
| P2.1 | EnvAdapter | process.env integration |
| P2.2 | MemoryAdapter | Testing support |
| P2.3 | CompositeAdapter | Multi-source merging |
| P2.4 | File watching | Auto-reload on change |

### Phase 3: Service Layer (Week 3)

| Task | Description | Deliverable |
|------|-------------|-------------|
| P3.1 | ConfigService | Load, get, reload |
| P3.2 | Validation | Zod integration |
| P3.3 | Error handling | Structured errors |
| P3.4 | Change events | Subscription API |

### Phase 4: Polish (Week 4)

| Task | Description | Deliverable |
|------|-------------|-------------|
| P4.1 | Testing | 90% coverage |
| P4.2 | Documentation | VitePress docs site |
| P4.3 | Examples | Usage patterns |
| P4.4 | Publish | npm @phenotype/config-ts |

---

## Resources

| Role | Allocation |
|------|------------|
| TypeScript Engineer | 0.5 FTE |

---

## Success Criteria

- [ ] All adapters tested
- [ ] 90% code coverage
- [ ] Full TypeScript types
- [ ] Published to npm
- [ ] Documentation complete
