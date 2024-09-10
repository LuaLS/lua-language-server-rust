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
    print(d:bpop())
end