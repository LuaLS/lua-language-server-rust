local fs = require "bee.filesystem"

local p = fs.path("resources/testmain.lua")

print(p / "aaaaa")

local thread = require "bee.thread"
thread.newchannel("hello")
local hello = thread.channel("hello")
print(hello)
hello:push("world", "yes", "no", 1, 2, 3)

local a, b, c, d, e, f = hello:pop()
print(a, b, c, d, e, f)