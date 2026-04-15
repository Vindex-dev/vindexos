return {
  "xiyaowong/nvim-transparent",
  lazy = false,
  priority = 1000,
  config = function()
    vim.opt.fillchars = { eob = " " }
    require("transparent").setup({
      enable = true,
      groups = {
        "Normal",
        "NormalNC",
        "NormalFloat",
        "SignColumn",
        "StatusLine",
        "StatusLineNC",
        "TabLine",
        "TabLineFill",
        "Pmenu",
        "CursorLine",
        "CursorLineNr",
        "LineNr",
        "WinBar",
        "WinBarNC",
        "EndOfBuffer",
      },
    })
  end,
}
