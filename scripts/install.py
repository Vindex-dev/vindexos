import json
import subprocess
import sys

CONFIG_PATH = "install_config.json"
MOUNT = "/mnt"

def run(cmd, **kwargs):
    subprocess.run(cmd, shell=True, check=True, **kwargs)

def chroot(cmd):
    run(f"arch-chroot {MOUNT} /bin/bash -c \"{cmd}\"")

def progress(percent, msg):
    print(f"PROGRESS:{percent}:{msg}", flush=True)

def log(msg):
    print(f"LOG:{msg}", flush=True)

with open(CONFIG_PATH) as f:
    cfg = json.load(f)

USERNAME   = cfg["username"]
PASSWORD   = cfg["password"]
HOSTNAME   = cfg["hostname"]
TIMEZONE   = cfg["timezone"]
LOCALE     = cfg["locale"]
LOCALE2    = cfg.get("locale2")
ROOT_DISK  = cfg["root_disk"]
HOME_DISK  = cfg.get("home_disk")
WIFI_SSID  = cfg.get("wifi_ssid")
WIFI_PASS  = cfg.get("wifi_pass", "")

def part(disk, n):
    if "nvme" in disk or "mmcblk" in disk:
        return f"/dev/{disk}p{n}"
    return f"/dev/{disk}{n}"

def detect_ucode():
    try:
        cpu_info = open("/proc/cpuinfo").read()
        if "GenuineIntel" in cpu_info:
            return "intel-ucode"
        elif "AuthenticAMD" in cpu_info:
            return "amd-ucode"
    except:
        pass
    return None

def detect_firmware():
    try:
        vendor = open("/sys/class/dmi/id/sys_vendor").read().strip().lower()
        if any(v in vendor for v in ["virtualbox", "qemu", "vmware", "innotek"]):
            return "linux-firmware"
    except:
        pass
    return "linux-firmware"

progress(5, "Checking internet connection...")
try:
    run("ping -c 1 archlinux.org -W 5 > /dev/null 2>&1")
except subprocess.CalledProcessError:
    print("ERROR:No internet connection", flush=True)
    sys.exit(1)

progress(8, "Updating mirrors with reflector...")
run("pacman -Sy --noconfirm reflector 2>/dev/null || true")
run("reflector --latest 20 --sort rate --save /etc/pacman.d/mirrorlist 2>/dev/null || true")

progress(10, f"Partitioning /dev/{ROOT_DISK}...")
run(f"parted /dev/{ROOT_DISK} --script mklabel gpt")
run(f"parted /dev/{ROOT_DISK} --script mkpart ESP fat32 1MiB 513MiB")
run(f"parted /dev/{ROOT_DISK} --script set 1 esp on")
run(f"parted /dev/{ROOT_DISK} --script mkpart primary ext4 513MiB 100%")

progress(20, "Formatting partitions...")
run(f"mkfs.fat -F32 {part(ROOT_DISK, 1)}")
run(f"mkfs.ext4 -F {part(ROOT_DISK, 2)}")

if HOME_DISK and HOME_DISK != ROOT_DISK:
    log(f"Formatting home disk /dev/{HOME_DISK}...")
    run(f"mkfs.ext4 -F /dev/{HOME_DISK}")

progress(25, "Mounting partitions...")
run(f"umount -R {MOUNT} 2>/dev/null || true")
run(f"mount -t ext4 {part(ROOT_DISK, 2)} {MOUNT}")
run(f"mount --mkdir {part(ROOT_DISK, 1)} {MOUNT}/boot")

if HOME_DISK and HOME_DISK != ROOT_DISK:
    run(f"mount --mkdir /dev/{HOME_DISK} {MOUNT}/home")

ucode = detect_ucode()
ucode_pkg = ucode if ucode else ""
firmware = detect_firmware()
progress(30, "Installing base system (this may take a while)...")
run(f"pacstrap -K {MOUNT} base linux-zen linux-zen-headers {firmware} "
    f"{ucode_pkg} base-devel git vim networkmanager sudo")

progress(55, "Generating fstab...")
run(f"genfstab -U {MOUNT} >> {MOUNT}/etc/fstab")

progress(60, f"Setting timezone {TIMEZONE}...")
chroot(f"ln -sf /usr/share/zoneinfo/{TIMEZONE} /etc/localtime")
chroot("hwclock --systohc")

progress(65, "Configuring locale...")
locale_line = LOCALE.split()[0]
run(f"sed -i 's/^#{locale_line}/{locale_line}/' {MOUNT}/etc/locale.gen")
if LOCALE2:
    locale2_line = LOCALE2.split()[0]
    run(f"sed -i 's/^#{locale2_line}/{locale2_line}/' {MOUNT}/etc/locale.gen")
chroot("locale-gen")
run(f"echo 'LANG={locale_line}' > {MOUNT}/etc/locale.conf")

progress(70, "Setting hostname...")
run(f"echo '{HOSTNAME}' > {MOUNT}/etc/hostname")

progress(75, "Enabling NetworkManager...")
chroot("systemctl enable NetworkManager")

if WIFI_SSID:
    progress(78, f"Configuring WiFi {WIFI_SSID}...")
    run(f"mkdir -p {MOUNT}/etc/NetworkManager/system-connections")
    if WIFI_PASS:
        security_section = f"""
[wifi-security]
key-mgmt=wpa-psk
psk={WIFI_PASS}
"""
    else:
        security_section = ""

    nm_profile = f"""[connection]
id={WIFI_SSID}
type=wifi

[wifi]
ssid={WIFI_SSID}
{security_section}
[ipv4]
method=auto

[ipv6]
method=auto
"""
    with open(f"{MOUNT}/etc/NetworkManager/system-connections/{WIFI_SSID}.nmconnection", "w") as f:
        f.write(nm_profile)
    run(f"chmod 600 '{MOUNT}/etc/NetworkManager/system-connections/{WIFI_SSID}.nmconnection'")

progress(80, f"Creating user {USERNAME}...")
chroot(f"useradd -m -G wheel -s /bin/bash {USERNAME}")
run(f"echo '{USERNAME}:{PASSWORD}' | arch-chroot {MOUNT} chpasswd")
run(f"echo 'root:{PASSWORD}' | arch-chroot {MOUNT} chpasswd")
chroot("sed -i 's/^# %wheel ALL=(ALL:ALL) ALL/%wheel ALL=(ALL:ALL) ALL/' /etc/sudoers")

progress(85, "Installing systemd-boot...")
chroot("bootctl install")

root_uuid = subprocess.check_output(
    f"blkid -s UUID -o value {part(ROOT_DISK, 2)}", shell=True
).decode().strip()

loader_conf = "default arch\ntimeout 3\neditor no\n"
run(f"mkdir -p {MOUNT}/boot/loader/entries")
with open(f"{MOUNT}/boot/loader/loader.conf", "w") as f:
    f.write(loader_conf)

initrd_lines = f"initrd  /{ucode}.img\ninitrd  /initramfs-linux-zen.img" if ucode else "initrd  /initramfs-linux-zen.img"

entry_conf = f"""title   Arch Linux (zen)
linux   /vmlinuz-linux-zen
{initrd_lines}
options root=UUID={root_uuid} rw
"""
with open(f"{MOUNT}/boot/loader/entries/arch.conf", "w") as f:
    f.write(entry_conf)

progress(95, "Running post-install setup...")
import shutil, os
shutil.copy("scripts/post_install.sh", f"{MOUNT}/root/post_install.sh")
run(f"chmod +x {MOUNT}/root/post_install.sh")
run(f"arch-chroot {MOUNT} /bin/bash /root/post_install.sh")

progress(98, "Unmounting...")
run(f"umount -R {MOUNT}")

progress(100, "Installation complete! You can reboot now.")
