#include "luajit-patch.h"

int common_lua_rawgeti(lua_State* L, int idx, int n)
{
#if LUA_VERSION_NUM >= 503
	lua_rawgeti(L, idx, n);
#else
	lua_rawgeti(L, idx, n);
	return lua_type(L, -1);
#endif
}

int common_lua_isinteger(lua_State* L, int idx)
{
#if LUA_VERSION_NUM >= 503
	return lua_isinteger(L, idx);
#else
	if (lua_type(L, idx) == LUA_TNUMBER) {
        lua_Number n = lua_tonumber(L, idx);
        return n == (lua_Integer)n;
    }
    return 0;
#endif
}

int common_lua_rawgetp(lua_State *L, int idx, const void *p)
{
#if LUA_VERSION_NUM >= 503
    return lua_rawgetp(L, idx, p);
#else
    lua_Integer key = (lua_Integer)p;
    lua_pushinteger(L, key);
	lua_rawget(L, idx);
	return lua_type(L, -1);
#endif
}

void common_lua_rawsetp(lua_State *L, int idx, const void *p)
{
#if LUA_VERSION_NUM >= 503
    lua_rawsetp(L, idx, p);
#else
    lua_Integer key = (lua_Integer)p;
    lua_pushinteger(L, key);
    lua_insert(L, -2);
    lua_rawset(L, idx);
#endif
}

size_t common_lua_rawlen(lua_State *L, int idx)
{
#if LUA_VERSION_NUM >= 503
    return lua_rawlen(L, idx);
#else
    return lua_objlen(L, idx);
#endif
}
