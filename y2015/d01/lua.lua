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
local file, err = io.open("y2015/d01/input")
if not file then
	error(err)
end

local line = file:read()
file:close()

local floor = 0
local position = -1
for i = 1, string.len(line) do
	local c = line:sub(i, i)
	if c == "(" then
		floor = floor + 1
	elseif c == ")" then
		floor = floor - 1
	end

	if position == -1 and floor < 0 then
		position = i
	end
end

return floor, position
