# Scripts path

By default, Omakure reads scripts from:

- `~/Documents/omakure-scripts` (Linux/macOS)
- `%USERPROFILE%\Documents\omakure-scripts` (Windows)

## Change the default path

Set `OMAKURE_SCRIPTS_DIR` before running `omakure`.

Linux/macOS:

```bash
export OMAKURE_SCRIPTS_DIR=/path/to/scripts
omakure
```

Windows (PowerShell):

```powershell
$env:OMAKURE_SCRIPTS_DIR = "C:\path\to\scripts"
omakure
```

## Development note

In debug builds, the app will use the repo `scripts/` folder if it exists. You can still override it with `OMAKURE_SCRIPTS_DIR`.
