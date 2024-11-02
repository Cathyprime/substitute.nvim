local subfunc = [[v:lua.require'substitute'.sub()]]
local opfunc = [[v:lua.require'substitute'.replace]]

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
        return "Query replace in region (" .. cache.query .. " -> " .. cache.replace .. "): "
    end
end

---@return string
local function replace_prompt()
    if cache.replace == "" then
        return "Query replace " .. cache.query .. " with: "
    else
        return "Query replacing " .. cache.query .. " with " .. cache.replace .. ": "
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

local function do_visually()
    local utils = require("substitute.utils")
    if not do_query() then
        return "<esc>"
    end
    if not do_replace() then
        return "<esc>"
    end
    return string.format([[:s#%s#\=%s#gc<cr>]], utils.find_regex(cache.query), subfunc)
end

return {
    input = input,
    query_prompt = query_prompt,
    replace_prompt = replace_prompt,
    do_query = do_query,
    do_replace = do_replace,
    do_visually = do_visually,
    cache_query = function()
        return cache.query
    end,
    cache_target = function()
        return cache.target
    end,
}
