local thread = require 'bee.thread'

local d = thread.channel('taskpad')
thread.thread([[
local thread = require 'bee.thread'
    print("hello world1")
local taskpad = thread.channel('taskpad')
    print("hello world2")
local counter = 0
while true do
    print("hello world")
    taskpad:push(counter)
    counter = counter + 1
    thread.sleep(100)
end
]])
-- thread.sleep(100)
print("hello world")
-- coroutine.yield()
print("thread 1", d, "hello world")
while true do
    print(d:bpop())
end