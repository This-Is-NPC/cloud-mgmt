# Development

## How to run in development

```bash
cargo run
```

Use the TUI to select a script, fill the fields, and run. Shortcuts:

- Ctrl+S: search scripts (background indexing)
- Alt+E: environment selector

In debug builds, the app will use the repo `scripts/` folder if it exists.
To override the scripts location, set `OMAKURE_SCRIPTS_DIR=/path/to/scripts`.
See `scripts-path.md` for default locations and overrides.

## Architecture (Rust code)

The code follows ports-and-adapters:

- `src/domain`: schema parsing and input normalization
- `src/ports`: traits for repository and runner
- `src/use_cases`: use case orchestration
- `src/adapters`: TUI, filesystem, script runners, system checks
- `src/workspace`: workspace layout helpers
- `src/history`: execution log persistence
