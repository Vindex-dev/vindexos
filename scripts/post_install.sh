#!/bin/bash
set -e

# Detect the non-root user created during install
USER_NAME=$(getent passwd | awk -F: '$3 >= 1000 && $3 < 65534 {print $1; exit}')
HOME_DIR="/home/$USER_NAME"

pacman -Sy --noconfirm reflector
reflector --latest 20 --sort rate --save /etc/pacman.d/mirrorlist

pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
pacman-key --lsign-key 3056513887B78AEB
pacman -U --noconfirm \
    'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst' \
    'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'

grep -q '\[chaotic-aur\]' /etc/pacman.conf || cat >> /etc/pacman.conf << 'EOF'

[chaotic-aur]
Include = /etc/pacman.d/chaotic-mirrorlist
EOF

pacman -Sy --noconfirm
if ! command -v yay &>/dev/null; then
    pacman -S --noconfirm --needed git base-devel
    sudo -u "$USER_NAME" bash -c "
        cd /tmp
        git clone https://aur.archlinux.org/yay-bin.git
        cd yay-bin
        makepkg -si --noconfirm
    "
fi

mkdir -p /etc/systemd/system/getty@tty1.service.d
cat > /etc/systemd/system/getty@tty1.service.d/autologin.conf << EOF
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin $USER_NAME --noclear %I \$TERM
EOF

pacman -Syu --noconfirm \
    mangowm alacritty nushell neovim tmux \
    mpd firefox rofi bash nemo btop \
    cargo rustup python cmake mpv \
    grim git slurp qview nvtop \
    pipewire pipewire-pulse wireplumber \
    xdg-user-dirs ttf-jetbrains-mono-nerd \
    noto-fonts noto-fonts-emoji stow

chsh -s /usr/bin/nu "$USER_NAME"

sudo -u "$USER_NAME" bash -c "
    cd $HOME_DIR
    git clone https://github.com/S1rEx1/dotfiles .dotfiles
    cd .dotfiles
    stow .
"

sed -i 's/^timeout.*/timeout 0/' /boot/loader/loader.conf

echo "Done. Reboot to apply."
