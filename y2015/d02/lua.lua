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
local paper = 0
local ribbon = 0
for line in io.lines("y2015/d02/input") do
	local l, w, h = string.match(line, "(%d+)x(%d+)x(%d+)")
	l, w, h = tonumber(l), tonumber(w), tonumber(h)
	local lw, wh, hl = l * w, w * h, h * l
	paper = paper + 2 * (lw + wh + hl) + math.min(lw, wh, hl)
	ribbon = ribbon + (2 * (l + w + h - math.max(l, w, h))) + (l * w * h)
end

return paper, ribbon
