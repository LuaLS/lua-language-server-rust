/*
 * patch for luajit by CppCXY
 */

#include <lua.h>
#include <lauxlib.h>
#include <string.h>

// Returns the number of characters in the UTF-8 string `s`
// that start between byte position `i` and `j` (both included).
// The default for `i` and `j` is to consider all characters in the string.
// For negative indices, it starts counting from the end of the string.
// If `lax` is true, the function returns the number of characters in the string,
// even if some of them are invalid.
// Invalid characters are always counted as one character.
// signature: utf8.len(s [, i [, j [, lax]]])
// signature (s, [i], [j], [lax])
int luajit_utf8_len(lua_State *L)
{
    size_t len;
    const char *s = luaL_checklstring(L, 1, &len);
    lua_Integer i = luaL_optinteger(L, 2, 1);
    lua_Integer j = luaL_optinteger(L, 3, len);
    int lax = lua_toboolean(L, 4);

    // Adjust negative indices
    if (i < 0)
        i += len + 1;
    if (j < 0)
        j += len + 1;

    // Clamp indices to the string boundaries
    if (i < 1)
        i = 1;
    if (j > (lua_Integer)len)
        j = len;
    if (i > j)
    {
        lua_pushinteger(L, 0);
        return 1;
    }

    size_t start = i - 1;
    size_t end = j - 1;
    size_t count = 0;

    // Traverse the string to count characters
    for (size_t p = start; p <= end;)
    {
        if ((s[p] & 0xC0) != 0x80)
        {
            count++;
        }
        if (!lax && (s[p] & 0xC0) == 0x80)
        {
            // Invalid UTF-8 sequence
            p++;
            continue;
        }
        p++;
    }

    lua_pushinteger(L, count);
    return 1;
}

// signature (s, n, [i])
int luajit_utf8_offset(lua_State *L)
{
    // Get the string and the integer n from the Lua stack
    size_t len;
    const char *s = luaL_checklstring(L, 1, &len);
    lua_Integer n = luaL_checkinteger(L, 2);
    lua_Integer i = luaL_optinteger(L, 3, 1);

    // Adjust the starting index to be 0-based
    if (i < 1)
        i = 1;
    size_t p = i - 1;

    // Traverse the string to find the byte offset of the nth UTF-8 character
    lua_Integer count = 0;
    while (p < len)
    {
        // Check if the current byte is the start of a UTF-8 character
        if ((s[p] & 0xC0) != 0x80)
        {
            count++;
            if (count == n)
            {
                lua_pushinteger(L, p + 1); // Lua uses 1-based indexing
                return 1;
            }
        }
        p++;
    }

    // If we reach here, it means the nth character was not found
    lua_pushnil(L);
}

// Receives zero or more integers,
// converts each one to its corresponding UTF-8 byte sequence and returns a string with the concatenation of
// all these sequences.
int luajit_utf8_char(lua_State *L)
{
    int n = lua_gettop(L); // Number of arguments
    luaL_Buffer b;
    luaL_buffinit(L, &b);

    for (int i = 1; i <= n; i++)
    {
        lua_Integer code = luaL_checkinteger(L, i);
        if (code < 0x80)
        {
            // 1-byte sequence
            luaL_addchar(&b, (char)code);
        }
        else if (code < 0x800)
        {
            // 2-byte sequence
            luaL_addchar(&b, (char)(0xC0 | (code >> 6)));
            luaL_addchar(&b, (char)(0x80 | (code & 0x3F)));
        }
        else if (code < 0x10000)
        {
            // 3-byte sequence
            luaL_addchar(&b, (char)(0xE0 | (code >> 12)));
            luaL_addchar(&b, (char)(0x80 | ((code >> 6) & 0x3F)));
            luaL_addchar(&b, (char)(0x80 | (code & 0x3F)));
        }
        else if (code < 0x110000)
        {
            // 4-byte sequence
            luaL_addchar(&b, (char)(0xF0 | (code >> 18)));
            luaL_addchar(&b, (char)(0x80 | ((code >> 12) & 0x3F)));
            luaL_addchar(&b, (char)(0x80 | ((code >> 6) & 0x3F)));
            luaL_addchar(&b, (char)(0x80 | (code & 0x3F)));
        }
        else
        {
            return luaL_error(L, "invalid UTF-8 code point");
        }
    }

    luaL_pushresult(&b);
    return 1;
}

// Helper function to decode a single UTF-8 character
static int luajit_utf8_decode(const char *s, int *len)
{
    unsigned char c = s[0];
    if (c < 0x80)
    {
        *len = 1;
        return c;
    }
    else if (c < 0xE0)
    {
        *len = 2;
        return ((c & 0x1F) << 6) | (s[1] & 0x3F);
    }
    else if (c < 0xF0)
    {
        *len = 3;
        return ((c & 0x0F) << 12) | ((s[1] & 0x3F) << 6) | (s[2] & 0x3F);
    }
    else
    {
        *len = 4;
        return ((c & 0x07) << 18) | ((s[1] & 0x3F) << 12) | ((s[2] & 0x3F) << 6) | (s[3] & 0x3F);
    }
}

// Returns the codepoints (as integers) from all characters
// in `s` that start between byte position `i` and `j` (both included).
// signature (s [i], [j], [lax]) -> multiple integer values
int luajit_utf8_codepoint(lua_State *L)
{
    size_t len;
    const char *s = luaL_checklstring(L, 1, &len);
    lua_Integer i = luaL_optinteger(L, 2, 1);
    lua_Integer j = luaL_optinteger(L, 3, len);
    int lax = lua_toboolean(L, 4);

    // Adjust negative indices
    if (i < 0)
        i += len + 1;
    if (j < 0)
        j += len + 1;

    // Clamp indices to the string boundaries
    if (i < 1)
        i = 1;
    if (j > (lua_Integer)len)
        j = len;
    if (i > j)
    {
        lua_pushnil(L);
        return 1;
    }

    size_t pos = i - 1;
    int char_len;
    int codepoint = luajit_utf8_decode(s + pos, &char_len);

    if (!lax && (char_len == 1 && (s[pos] & 0x80) != 0))
    {
        lua_pushnil(L);
        return 1;
    }

    lua_pushinteger(L, codepoint); // Push the first code point

    if (i == j)
    {
        return 1; // Return the single code point
    }

    int count = 1;
    pos += char_len;
    while (pos < (size_t)j)
    {
        codepoint = luajit_utf8_decode(s + pos, &char_len);
        if (!lax && (char_len == 1 && (s[pos] & 0x80) != 0))
        {
            lua_pushnil(L);
            return 1;
        }
        lua_pushinteger(L, codepoint); // Push the code point
        count++;
        pos += char_len;
    }

    return count; // Return the number of code points
}

// Iterator function
static int utf8_codes_iter(lua_State *L, int lax)
{
    size_t len;
    const char *s = luaL_checklstring(L, 1, &len);
    int pos = luaL_checkinteger(L, 2);

    if (pos >= (int)len)
    {
        return 0; // End of iteration
    }

    int char_len;
    int codepoint = luajit_utf8_decode(s + pos, &char_len);

    if (!lax && (char_len == 1 && (s[pos] & 0x80) != 0))
    {
        return luaL_error(L, "invalid UTF-8 byte sequence");
    }

    lua_pushinteger(L, pos + 1);   // Next position
    lua_pushinteger(L, codepoint); // Code point
    return 2;
}

static int iter_codes_strict(lua_State *L)
{
    return utf8_codes_iter(L, 0);
}

static int iter_codes_lax(lua_State *L)
{
    return utf8_codes_iter(L, 1);
}

// Returns values so that the construction
// ```lua
// for p, c in utf8.codes(s) do
//     body
// end
// ```
// will iterate over all UTF-8 characters in string s, with p being the position (in bytes) and c the code point of each character. It raises an error if it meets any invalid byte sequence.
// signature (s [, lax]) -> fun(s: string, p: integer):integer, integer
int luajit_utf8_codes(lua_State *L)
{
    int lax = lua_toboolean(L, 2);
    const char *s = luaL_checkstring(L, 1);
    lua_pushcfunction(L, lax ? iter_codes_lax : iter_codes_strict);
    lua_pushvalue(L, 1);
    lua_pushinteger(L, 0);
    return 3;
}