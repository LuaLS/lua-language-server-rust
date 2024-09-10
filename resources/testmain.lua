local thread = require 'bee.thread'

local d = thread.channel('taskpad')
thread.thread([[
local thread = require 'bee.thread'
    print("hello world1")
local taskpad = thread.channel('taskpad')
    print("hello world2")
local counter = 0
taskpad:push("hello world", "hello world2", counter)
]])
while true do
    local a, err, c, d = d:pop()
    if a then
        print(err, c, d)
    else
        print("sleep 1000")
        thread.sleep(1000)
    end
end