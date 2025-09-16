/* Copyright 2025 Philip Cronje
 *
 * This file is part of my Advent of Code solution repository. It is free software: you can
 * redistribute it and/or modify it under the terms of the GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this repository. If
 * not, see <https://www.gnu.org/licenses/>. */
alias Lua = void*;
alias LuaInteger = long;
alias LuaKContext = ptrdiff_t;
alias LuaKFunction = void*;

enum LUA_OK = 0;

enum LUA_TFUNCTION = 6;

extern (C):
void lua_close(Lua);
int lua_gettop(Lua);
int lua_isinteger(Lua, int);
int lua_isstring(Lua, int);
int lua_pcallk(Lua, int, int, int, LuaKContext, LuaKFunction);
void lua_settop(Lua, int);
LuaInteger lua_tointegerx(Lua, int, int*);
char* lua_tolstring(Lua, int, size_t*);
int lua_type(Lua, int);

int luaL_loadfilex(Lua, const char*, const char*);
Lua luaL_newstate();
void luaL_openlibs(Lua);
