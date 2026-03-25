# Strict-Mode Command Compatibility Matrix

Generated: 2026-03-03

Scope: template-sync adopters audited for strict-mode command triad (format/lint/unit).

| Repo | Language | Format | Lint | Unit | Strict Ready |
|---|---|---|---|---|---|
| thegent | python | ruff format --check | ruff check | pytest -x | yes |
| cliproxyapi++ | go | gofmt -l | golangci-lint run | go test ./... | yes |
| agentapi-plusplus | go | gofmt -l | make lint | go test ./... | yes |
| bifrost-extensions | go | gofmt -l | golangci-lint run | go test ./... | yes |
| portage | python | ruff format --check | ruff check | pytest -x | yes |
| civ | rust | cargo fmt --check | cargo clippy --all-targets -- -D warnings | cargo test --lib | yes |
| heliosApp | node | npm run format | just lint | npm test | yes |
| heliosCLI | rust | cargo fmt --check | cargo clippy --all-targets -- -D warnings | cargo test --lib | yes |

## Notes
- strict mode requires all triad commands to resolve for each adopter.
- non-strict mode allows gaps but reports drift for remediation.
