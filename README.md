# Substitute.nvim
> simple to use search and replace interface for neovim

## requirements
- cargo
- make

## setup

The plugin is lazy loaded by default, there is no need to set it up yourself

`lazy.nvim` example
```lua
    {
        "Cathyprime/substitute.nvim",
        build = "make",
        config = function()
            require("substitute")
            vim.keymap.set("n", "gs", "<Plug>(substitute)")
            vim.keymap.set("n", "gss", "<Plug>(substitute-linewise)")
            vim.keymap.set("x", "gs", "<Plug>(substitute)")
        end
    }
```
