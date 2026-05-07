## Summary

Stabilize main branch with policy-gate workflow and documentation governance updates.

## Context

Stabilization effort to ensure main branch is production-ready with proper CI gates.

## Changes

- Add policy-gate workflow
- Mark FFI functions as unsafe with Safety documentation
- Mass injection of Phenotype governance and worktree policies

## Testing

```bash
# Verify CI passes
npm test
npm run lint
```
