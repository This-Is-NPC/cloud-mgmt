# CLI usage

## Doctor

```bash
omakure doctor
```

Alias: `omakure check`

## List scripts

```bash
omakure scripts
```

Lists scripts recursively across the workspace (including `.omaken`).

## Run a script without the TUI

```bash
omakure run .omaken/azure/rg-list-all
omakure run tools/cleanup
omakure run scripts/cleanup.py -- --force
```

## Init a new script template

```bash
omakure init my-script
omakure init tools/cleanup.py
```

See `how-to-create-a-script.md` for the step-by-step guide and templates.

## Config / env

```bash
omakure config
omakure env
```

TUI notes:

- The Environments screen shows a preview panel for the selected env file.
- Preview scroll: `PgUp` / `PgDn`, `Home` / `End`.
- See `environments.md` for details.

## Themes

```bash
omakure theme list
omakure theme set <name>
omakure theme preview <name>
omakure theme path
```

- Global theme config: `~/.config/omakure/config.toml` with `[theme] name = "..."`.
- Built-in themes are copied to `~/.config/omakure/themes/` on first use.
- Workspace override: add `[theme] name = "..."` to `omakure.toml`.

## Omaken flavors

```bash
omakure list
omakure install <git-url>
omakure install <git-url> --name my-flavor
```

## Shell completion

```bash
omakure completion bash
omakure completion zsh
omakure completion fish
omakure completion pwsh
```
