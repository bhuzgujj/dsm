# Gui
UI to manage datasets

## Setup
Requires:
- [Tauri 2.0](https://tauri.app/start/prerequisites/)

## Generate MSI (Windows)
### Manual
Build [cli](../cli/readme.md) first.

Afterward, build the gui

```powershell
npm run tauri build
```

### Script
The script `./build_msi.ps1` will execute the required command to generate the msi.
