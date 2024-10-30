local parser = require "lua.parser"

local t = [[
local t = 123
-- hhihih
]]

local t = parser.parse(t)
local root = t:getRoot()
for i, child in pairs(root:getDescendants()) do
    print(i, child.kindText, child:getText())
end

