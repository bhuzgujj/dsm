# Datasets Manager
- [Setup](#setup)
- [Graphical User Interface](./gui/readme.md)
- [Commandline Interface](./cli/readme.md)

# Setup
## Using nix-shell
Normal working shell:
```sh
nix-shell shell.nix
```

Warning, the musl version nix-shell takes a which to first startup
```sh
nix-shell musl.nix
```

## Manual
Requirements for [Tauri 2.0](https://tauri.app/start/prerequisites/)

Other requirements
- [Python v3.12.5](https://www.python.org/downloads/release/python-3125/)
