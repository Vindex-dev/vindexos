# VindexOS

TUI installer for Arch Linux, written in Rust.

## Userspace preview(everything is gruvbox)
### Holy blessed firefox
<img width="2560" height="1440" alt="image" src="https://github.com/user-attachments/assets/65ffe725-f278-4dd8-ac33-1104a24a6565" />
### Pre configured neovim
<img width="2545" height="1440" alt="image" src="https://github.com/user-attachments/assets/a18822d4-2c85-43b0-b59f-254ea133bc8e" />
### Configured MDP and magnowm
<img width="2560" height="1440" alt="image" src="https://github.com/user-attachments/assets/c471e785-ad9b-452b-93b9-8cb907a8118b" />


## Installation

Download and run in Arch Linux live environment:

```bash
curl -L https://github.com/Vindex-dev/vindexos/releases/latest/download/vindexos-release.tar.gz | tar -xz
sudo ./VindexOS
```

## Configuration

The installer will guide you through:
- Username and password
- Hostname
- WiFi network (optional)
- Timezone (fuzzy search)
- Locale (primary and optional secondary)
- Disk partitioning (root and optional separate home)

After configuration, the installation runs automatically with real-time progress.

## Development

Build from source:

```bash
git clone https://github.com/Vindex-dev/vindexos.git
cd vindexos
cargo build --release
```

## Requirements

- UEFI system
- Internet connection
- Python 3 (for installation script)

## License

MIT

