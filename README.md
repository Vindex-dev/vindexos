# VindexOS

TUI installer for Arch Linux, written in Rust.

## Features

- Interactive TUI with fuzzy search for timezone and locale selection
- WiFi configuration during installation
- Disk partitioning with separate root and home support
- systemd-boot bootloader
- Real-time installation progress with logs
- Automatic CPU microcode detection (Intel/AMD)
- Virtual machine detection (skips firmware in VMs)

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

