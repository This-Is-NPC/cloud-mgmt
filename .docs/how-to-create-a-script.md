# How to create a Script

You can generate a starter script with:

```bash
omakure init my-script
```

Pass an extension to choose the template (`.bash`, `.sh`, `.ps1`, `.py`). If omitted, `.bash` is used.

## Step by step

1) Copy the template below to `~/Documents/omakure-scripts/my-script.bash` (Windows: `%USERPROFILE%\Documents\omakure-scripts\my-script.bash`). Use `.ps1` or `.py` for other runtimes.
2) Edit the schema JSON (name, description, and fields).
3) Adjust defaults and argument parsing.
4) Write the main logic.
5) Test:
   - `SCHEMA_MODE=1 bash scripts/my-script.bash` (should print valid JSON)
   - `bash scripts/my-script.bash --your-param value`
   - `cargo run` and select the script in the TUI

## Script anatomy

A script needs 4 clear blocks:

1) **Schema**: JSON that the TUI uses to know which fields to ask for.
2) **Defaults**: variables with initial values.
3) **Args + prompts**: reads `--param value` and asks if missing.
4) **Main**: script logic.

## Schema fields (JSON)

- `Name`: script identifier.
- `Description`: short description of what it does.
- `Fields`: list of fields for the TUI.

For each field in `Fields`:

- `Name`: internal field name.
- `Prompt`: text shown to the user.
- `Type`: `string`, `number`, or `bool`.
- `Order`: display order.
- `Required`: `true` or `false`.
- `Arg`: CLI argument name (e.g., `--target`).
- `Default`: default value (optional).
- `Choices`: list of allowed values (optional).

## Simple template (copy and paste)

```bash
#!/usr/bin/env bash
set -euo pipefail

# 1) Schema for the TUI
if [[ "${SCHEMA_MODE:-}" == "1" ]]; then
  cat <<'JSON'
{
  "Name": "my_script",
  "Description": "Describe what this script does.",
  "Fields": [
    {
      "Name": "target",
      "Prompt": "Target (optional)",
      "Type": "string",
      "Order": 1,
      "Required": false,
      "Arg": "--target"
    }
  ]
}
JSON
  exit 0
fi

# 2) Defaults
TARGET=""

# 3) Args + prompts
prompt_if_empty() {
  local var_name="$1"
  local label="$2"
  local value="${!var_name:-}"
  if [[ -z "${value}" ]]; then
    read -r -p "${label}: " value
    printf -v "${var_name}" '%s' "${value}"
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET="${2:-}"
      shift 2
      ;;
    *)
      echo "Unknown arg: $1" >&2
      exit 1
      ;;
  esac
done

prompt_if_empty TARGET "Target (optional)"

# 4) Main
printf "Running with target=%s\n" "${TARGET}"
```
