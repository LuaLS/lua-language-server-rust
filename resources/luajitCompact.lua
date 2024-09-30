table.unpack = unpack

table.pack = function(...)
    return { n = select('#', ...), ... }
end

function defer(toBeClosed, callback)
    local ctype = type(toBeClosed)
    if ctype == 'table' then
        local meta = getmetatable(toBeClosed)
        if meta and meta.__close then
            local ok, result = xpcall(callback, log.error)
            meta.__close(toBeClosed)
            if ok then
                return result
            end
        else
            local ok, result = xpcall(callback, log.error)
            if ok then
                return result
            end
        end
    elseif ctype == 'function' then
        local ok, result = xpcall(callback, log.error)
        toBeClosed()
        if ok then
            return result 
        end
    end
end

math.maxinteger = 0x7FFFFFFFLL
function math.tointeger(x)
    if type(x) ~= "number" then
        return nil
    end

    local int = x >= 0 and math.floor(x) or math.ceil(x)

    if int == x then
        return int
    else
        return nil
    end
end