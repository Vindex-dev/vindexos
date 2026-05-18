# post install script to set up configs

sudo pacman -S < pkglist.txt
git clone https://S1rEx1/dotfiles
cd dotfiles
stow .

