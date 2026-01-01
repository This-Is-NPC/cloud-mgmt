# Lua widgets (index.lua)

Folders can include an `index.lua` file to render a widget in the TUI header panel.

## Expected shape

Omakure loads `index.lua` and expects a table with `title` and `lines` (list of strings). You can provide it in three ways:

1) Return the table:

```lua
return {
  title = "Azure",
  lines = {
    "rg-prod",
    "rg-staging",
  },
}
```

2) Set a global `widget` table:

```lua
widget = {
  title = "Azure",
  lines = {
    "rg-prod",
    "rg-staging",
  },
}
```

3) Set global `title` and `lines` directly:

```lua
title = "Azure"
lines = {
  "rg-prod",
  "rg-staging",
}
```

## Notes

- `lines` must be a list/array of strings.
- If the file exists but does not provide `title` and `lines`, Omakure will show a load error.
