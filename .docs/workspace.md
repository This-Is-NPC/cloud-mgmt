# Workspace layout

Omakure treats the workspace as a filesystem score:

```
omakure-scripts/
├── .omaken/        # Curated flavors managed by Omakure
│   └── azure/
│       ├── index.lua   # Optional folder widget
│       ├── rg-list-all.bash
│       ├── rg-details.bash
│       └── rg-delete.bash
│   └── envs/       # Environment defaults (active file listed in .omaken/envs/active)
│       ├── active
│       └── env_template.conf
├── .history/       # Execution logs
└── omakure.toml    # Optional workspace config
```

If a folder includes `index.lua`, Omakure renders it in the TUI header panel.

Environment defaults live in `.omaken/envs/*.conf`. Use the TUI (Alt+E) to switch the active file.
Defaults are applied by matching field names (case-insensitive) to `key=value` pairs.
See `environments.md` for usage details.

The `.history/` folder stores local run logs and is ignored by git.
