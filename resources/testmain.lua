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

local time = require "bee.time"
print(time.time())
print(time.monotonic())

local windows = require "bee.windows"
for k, v in pairs(windows) do
    print(k, v)
end
windows.filemode(io.stdin, 'b')

local socket = require "bee.socket"
local select = require "bee.select"
local selector = select.create()
thread.sleep(100)
print("sleep complete")
local fd = socket.create("tcp")
fd:bind("127.0.0.1", 9988)
print("bind complete")
fd:listen()
print("listen complete")
-- local cfd = fd:accept()
-- print(cfd)

-- selector:event_add(fd, 1, function()
--     print("listener fd", fd)
--     local cfd = fd:accept()
--     print("accept a connection", cfd)
-- end)

-- local cfd = fd:accept()
-- print(cfd)

-- while true do
--     for func, event in selector:wait(1000) do
--         print("func", func, "event", event)
--         if func then
--             func(event)
--         end
--     end
-- end


