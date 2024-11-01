local utils = require("substitute.utils")

local opfunc = [[v:lua.require'substitute'.replace]]
local subfunc = [[v:lua.require'substitute'.sub()]]

local opts = {
    silent = true,
    expr = true
}

local cache = {
    query = "",
    replace = "",
    target = "",
}

local function input(opts)
    opts.cancelreturn = nil
    local ok, result = pcall(vim.fn.input, opts)
    if not ok then return end
    return result
end

---@return string
local function query_prompt()
    if cache.query == "" then
        return "Query replace in region: "
    else
        return "Query replace in region ("..cache.query.." -> "..cache.replace.."): "
    end
end

local function replace_prompt()
    if cache.replace == "" then
        return "Query replace "..cache.query.." with: "
    else
        return "Query replacing "..cache.query.." with "..cache.replace..": "
    end
end

local function do_query()
    local query = input({
        prompt = query_prompt()
    })

    if not query then
        return false
    end
    cache.query = query ~= "" and query or cache.query
    return true
end

local function do_replace()
    local replace = input({
        prompt = replace_prompt()
    })

    if not replace then
        return false
    end

    cache.target = replace == "" and cache.target or replace
    cache.replace = replace ~= "" and replace or cache.replace
    return true
end

local function setup(linewise)
    vim.go.operatorfunc = opfunc
    if linewise then
        return "g@_"
    else
        return "g@"
    end
end

local function do_visually()
    if not do_query() then
        return "<esc>"
    end
    if not do_replace() then
        return "<esc>"
    end
    return string.format([[:s#%s#\=%s#gc<cr>]], utils.find_regex(cache.query), subfunc)
end

vim.keymap.set("n", "gs", function()
    return setup(false)
end, opts)

vim.keymap.set("n", "gss", function()
    return setup(true)
end, opts)

vim.keymap.set("x", "gs", do_visually, opts)

local function keys(cmd)
    vim.api.nvim_feedkeys(cmd, 'n', false)
end

local function keyc(cmd)
    vim.api.nvim_feedkeys(vim.keycode(cmd), 'n', false)
end

return {
    replace = function()
        if not do_query() then
            keyc "<esc>"
        end
        if not do_replace() then
            keyc "<esc>"
        end
        keys(string.format(vim.keycode[[:'[,']s#%s#\=%s#gc<cr>]], utils.find_regex(cache.query), subfunc))
        vim.go.operatorfunc = opfunc
    end,
    sub = function()
        local submatch = vim.fn.submatch(0)
        return utils.replace(submatch, cache.target)
    end
}
