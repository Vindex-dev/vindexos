$env.PROMPT_COMMAND = {
    let path = (pwd | str replace $env.HOME "~")
    $"(ansi cyan)($path)(ansi reset)\n(ansi white)❯ (ansi reset)"
}
$env.PROMPT_COMMAND_RIGHT = { "" }
$env.PROMPT_INDICATOR = ""
$env.config.buffer_editor = "nvim"


$env.LANG = "en_US.UTF-8"
$env.LC_ALL = "en_US.UTF-8"
