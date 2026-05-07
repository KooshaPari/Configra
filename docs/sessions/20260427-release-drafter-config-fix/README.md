# Release Drafter config fix

## Goal

Fix the `Release Drafter failing on main` alert by adding the repo-local configuration file
that the reusable workflow expects.

## Scope

- Add `.github/release-drafter.yml` with a default release template.
- Leave the reusable workflow wrapper intact.

## Success criteria

- The release-drafter job can load a config file from the repo.
- The alert-sync issue can be re-evaluated after the patch is merged.
