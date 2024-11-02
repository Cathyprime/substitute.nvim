local subfunc = [[v:lua.require'substitute'.sub()]]
local opfunc = [[v:lua.require'substitute'.replace]]

local opts = {
    silent = true,
    expr = true
}

local function setup(linewise)
    vim.go.operatorfunc = opfunc
    if linewise then
        return "g@_"
    else
        return "g@"
    end
end

vim.keymap.set("n", "<Plug>(substitute)", function()
    return setup(false)
end, opts)

vim.keymap.set("n", "<Plug>(substitute-linewise)", function()
    return setup(true)
end, opts)

vim.keymap.set("x", "<Plug>(substitute)", require("substitute._internals").do_visually, opts)

local function keyc(cmd)
    vim.api.nvim_feedkeys(vim.keycode(cmd), 'n', false)
end

local function keys(cmd)
    vim.api.nvim_feedkeys(cmd, 'n', false)
end

return {
    replace = function()
        local utils = require("substitute.utils")
        local i = require("substitute._internals")
        if not i.do_query() then
            keyc "<esc>"
        end
        if not i.do_replace() then
            keyc "<esc>"
        end
        keys(string.format(vim.keycode [[:'[,']s#%s#\=%s#gc<cr>]], utils.find_regex(i.cache_query()), subfunc))
        vim.go.operatorfunc = opfunc
    end,
    sub = function()
        local utils = require("substitute.utils")
        local i = require("substitute._internals")
        local submatch = vim.fn.submatch(0)
        return utils.replace(submatch, i.cache_target())
    end
}
