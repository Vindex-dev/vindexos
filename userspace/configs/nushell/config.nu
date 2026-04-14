#=============#
# AUR ALIASES #
#=============#
alias i = yay -S --noconfirm --needed
alias r = yay -R
alias s = yay -Ss


#=============#
# GIT ALIASES #
#=============#
alias gc = git commit -am
alias gp = git push
alias gpl = git pull
alias ga = git add .


alias zj = zellij


#=============#
# VIM ALIASES #
#=============#
alias n = nvim
alias v = nvim
alias nano = nvim


#=============#
# PIP ALIASES #
#=============#
alias pi = pip install --break-system-packages
alias pir = pip install -r requirements.txt --break-system-packages

alias o = bat
$env.config = {
  show_banner: false
}
$env.PATH = ($env.PATH | append "/home/sirex/.cargo/bin")
