# Harden TruffleHog secrets workflow

## Goal
Remove floating tool installs and add explicit least-privilege token permissions for Configra secrets scanning.

## Acceptance criteria
- TruffleHog workflow no longer installs the scanner with `@latest`.
- TruffleHog workflow declares explicit read-only GitHub token permissions.
- Existing pinned action references remain unchanged.
- Workflow hygiene scanner reports no floating `@latest` refs in the TruffleHog workflow.
