# Installation

## Installation (one command, GitHub Releases)

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/This-Is-NPC/omakure/main/install.sh | bash -s -- --repo This-Is-NPC/omakure
```

Windows (PowerShell):

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/This-Is-NPC/omakure/main/install.ps1 | iex"
```

Then run:

```bash
omakure
```

The installation creates the scripts folder at `~/Documents/omakure-scripts` (Windows: `%USERPROFILE%\Documents\omakure-scripts`).

## Update

```bash
omakure update
```

Linux/macOS requires `curl` (or `wget`) and `tar` for the update flow. Windows uses PowerShell.
The update also syncs new scripts from the repo without overwriting existing files.

Optional overrides:

```bash
omakure update --version v0.1.1 --repo This-Is-NPC/omakure
```

## Uninstall

```bash
omakure uninstall
```

To remove the scripts folder as well:

```bash
omakure uninstall --scripts
```

## Install a specific version

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/This-Is-NPC/omakure/main/install.sh | VERSION=v0.1.1 bash -s -- --repo This-Is-NPC/omakure
```

Windows (PowerShell):

```powershell
$env:REPO = "This-Is-NPC/omakure"
$env:VERSION = "v0.1.1"
irm https://raw.githubusercontent.com/This-Is-NPC/omakure/main/install.ps1 | iex
```

## Install from source (optional)

```bash
bash install-from-source.sh
```
