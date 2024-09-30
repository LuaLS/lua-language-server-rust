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