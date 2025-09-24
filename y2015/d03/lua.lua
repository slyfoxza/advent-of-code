--[[
Copyright 2025 Philip Cronje

This file is part of my Advent of Code solution repository. It is free software: you can
redistribute it and/or modify it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at your option) any later
version.

This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along with this repository. If
not, see <https://www.gnu.org/licenses/>.
--]]
local file, err = io.open("y2015/d03/input")
if not file then
	error(err)
end

local line = file:read()
file:close()

local function new_point(x, y)
	return { x = x, y = y }
end

local visited1, visited2 = {}, {}
local function insert_point(t, p)
	key = p.x .. "," .. p.y
	t[key] = true
end

local solo_santa_pos = new_point(0, 0)
insert_point(visited1, solo_santa_pos)

local santa_pos = new_point(0, 0)
local robo_pos = new_point(0, 0)
local current = robo_pos

for i = 1, string.len(line) do
	if rawequal(current, robo_pos) then
		current = santa_pos
	else
		current = robo_pos
	end

	local c = line:sub(i, i)
	if c == "^" then
		solo_santa_pos.y = solo_santa_pos.y - 1
		current.y = current.y - 1
	elseif c == "v" then
		solo_santa_pos.y = solo_santa_pos.y + 1
		current.y = current.y + 1
	elseif c == "<" then
		solo_santa_pos.x = solo_santa_pos.x - 1
		current.x = current.x - 1
	elseif c == ">" then
		solo_santa_pos.x = solo_santa_pos.x + 1
		current.x = current.x + 1
	end

	insert_point(visited1, solo_santa_pos)
	insert_point(visited2, current)
end

local function tlen(t)
	local n = 0
	for k, v in pairs(t) do
		n = n + 1
	end
	return n
end

return tlen(visited1), tlen(visited2)
