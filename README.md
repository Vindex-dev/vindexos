# VindexOS

TUI installer(currently just an installer) for Arch Linux, written in Rust.

## TODO frontend:
- [x] Username
- [x] Hostname
- [x] WiFi setup
- [ ] Timezone select
- [ ] Rice select
- [x] Disk partitioning
- [x] Save to json

## TODO backend:
we'll finish front first

## Usage

```bash
git clone git@github.com:Vindex-dev/vindexos.git; cd vindexos
cargo build --release #or cargo run
./target/release/vindexos #if cargo run then no need for this
```


## Config output

```json
{
  "username": "A.H.",
  "hostname": "komputr",
  "password": "516", //iykyk
  "password_confirm": "516",
  "wifi_ssid": "wifi soseda",
  "timezone": "Europe/Moscow",
  "root_disk": "nvme0n1",
  "home_disk": "sda"
}
```

