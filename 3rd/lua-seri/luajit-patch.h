/*
* patch for luajit by CppCXY
*/

#ifndef LUAJIT_PATCH_H
#define LUAJIT_PATCH_H

#include <lua.h>


// Patch for LuaJIT

int common_lua_rawgeti(lua_State *L, int idx, int n);

int common_lua_isinteger(lua_State *L, int idx);

int common_lua_rawgetp(lua_State *L, int idx, const void *p);

void common_lua_rawsetp(lua_State *L, int idx, const void *p);

size_t common_lua_rawlen(lua_State *L, int idx);

#endif