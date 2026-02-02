# Contributing

Thanks for helping improve Omakure! This guide explains how to get set up and submit changes.

## Getting started

1) Fork the repo and clone your fork.
2) Create a branch from `main` for your change.
3) Make focused changes and keep commits small.

## Requirements

- Rust toolchain (stable)
- Git
- `jq`
- Bash (for `.bash`/`.sh` scripts)
- PowerShell (optional, for `.ps1` scripts)
- Python (optional, for `.py` scripts)

## Run locally

```bash
cargo run
```

Debug builds use the repo `scripts/` folder if it exists. To override the scripts location, set:

```bash
OMAKURE_SCRIPTS_DIR=/path/to/scripts
```

## Tests

```bash
cargo test
```

## Pull requests

- Keep PRs focused on one topic.
- Update docs in `.docs/` when behavior changes.
- Include steps to validate your change if it affects the CLI or TUI.

## Release notes

The auto-release workflow requires a release notes file for the next version. If your change is
user-facing or should ship in the next release, add:

```
release-notes/vX.X.X.md
```

To determine `vX.X.X`, check the latest tag and bump the patch version (for example, `v0.1.5`
becomes `v0.1.6`).

## Helpful docs

- Development guide: `.docs/development.md`
- Usage: `.docs/usage.md`
- Workspace layout: `.docs/workspace.md`
- How to create a script: `.docs/how-to-create-a-script.md`
