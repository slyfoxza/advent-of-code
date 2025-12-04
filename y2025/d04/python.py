# Copyright 2025 Philip Cronje
#
# This file is part of my Advent of Code solution repository. It is free software: you can
# redistribute it and/or modify it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or (at your option) any later
# version.
#
# This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with this repository. If
# not, see <https://www.gnu.org/licenses/>.
from copy import deepcopy
import itertools

def remove_rolls(grid) -> int:
    w, h = len(grid[0]), len(grid)
    removed = 0
    for x, y in itertools.product(range(w), range(h)):
        if grid[y][x] != '@':
            continue

        adj_count = 0
        for dx, dy in itertools.product(range(-1, 2), range(-1, 2)):
            if dx == 0 and dy == 0:
                continue
            x1, y1 = x + dx, y + dy
            if x1 < 0 or x1 >= w or y1 < 0 or y1 >= h:
                continue

            if grid[y1][x1] != '.':
                adj_count += 1
            if adj_count >= 4:
                break
        if adj_count < 4:
            grid[y][x] = 'x'
            removed += 1

    for x, y in itertools.product(range(w), range(h)):
        if grid[y][x] == 'x':
            grid[y][x] = '.'
    return removed

def python_y2025_d04():
    grid = []
    with open("y2025/d04/input") as f:
        while len(line := f.readline().rstrip()) != 0:
            grid.append(list(line))
    grid2 = deepcopy(grid)

    accessible = remove_rolls(grid)

    total_removed = 0
    while (removed := remove_rolls(grid2)) != 0:
        total_removed += removed

    return (accessible, total_removed)

if __name__ == '__main__':
    print(*python_y2025_d04())
