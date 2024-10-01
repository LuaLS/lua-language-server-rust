local select = select
local type = type
local getmetatable = getmetatable
local xpcall = xpcall
local math_floor = math.floor
local math_ceil = math.ceil
local coroutine_resume = coroutine.resume
local coroutine_status = coroutine.status
local coroutine_running = coroutine.running
local coroutine_create = coroutine.create
local yield = coroutine.yield
local pcall = pcall
local unpack = unpack

table.unpack = unpack

table.pack = function(...)
    return { n = select('#', ...), ... }
end

math.maxinteger = 0x7FFFFFFFLL
local function tointeger(x)
    if type(x) ~= "number" then
        return nil
    end

    local int = x >= 0 and math_floor(x) or math_ceil(x)

    if int == x then
        return int
    else
        return nil
    end
end

math.tointeger = tointeger

function math.type(x)
    if type(x) == "number" then
        return tointeger(x) and "integer" or "number"
    else
        return nil
    end
end



local cancel_table = setmetatable({}, { __mode = 'kv' })

function coroutine.resume(co, ...)
    local results = { pcall(coroutine_resume, co, ...) }
    local ok = results[1]
    if not ok then
        local reason = results[2]
        if reason == "cancel" then
            return false, 'cannot resume dead coroutine'
        end
    end

    return ok, unpack(results, 2)
end

-- donot return 'dead' status
-- function coroutine.status(co)
--     return coroutine_status(co)
-- end

-- 要实现取消, 只能在协程内的调用通过错误让协程消亡
function coroutine.yield(...)
    if cancel_table[coroutine_running()] then
        error("cancel")
    end
    return yield(...)
end

function coroutine.close(co)
    if coroutine_status(co) == 'suspended' then
        cancel_table[co] = true
        return true
    end
    return false
end

function defer(toBeClosed, callback)
    local ctype = type(toBeClosed)
    local meta = getmetatable(toBeClosed)
    local ok, result

    ok, result = xpcall(callback, log.error)
    if meta and meta.__close then
        meta.__close(toBeClosed)
    elseif ctype == 'function' then
        toBeClosed()
    end

    if ok then
        return result
    end
end
