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
def python_y2025_d07():
    filename = 'y2025/d07/input'

    grid = []
    with open(filename) as f:
        while len(line := f.readline().rstrip()) != 0:
            line = list(line)
            if len(grid) != 0:
                for i in range(len(line)):
                    above = grid[-1][i]
                    if above == 'S':
                        line[i] = '|'
                    elif line[i] == '.' and above == '|':
                        line[i] = '|'
                    elif line[i] == '^' and above == '|':
                        line[i-1] = '|'
                        line[i+1] = '|'
            grid.append(line)

    n_splits = 0
    for i in range(1, len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] == '^' and grid[i-1][j] == '|':
                n_splits += 1

    grid = []
    with open(filename) as f:
        while len(line := f.readline().rstrip()) != 0:
            grid.append(list(line))

    ngrid = []
    for i in range(len(grid)):
        ngrid.append([0] * len(grid[i]))
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] == 'S':
                ngrid[i][j] = 1
            elif grid[i][j] == '^':
                ngrid[i][j] = 0
                ngrid[i][j-1] += ngrid[i-1][j]
                ngrid[i][j+1] += ngrid[i-1][j]
            elif ngrid[i-1][j] != 0:
                ngrid[i][j] += ngrid[i-1][j]
    n_timelines = sum(ngrid[-1])

    return (n_splits, n_timelines)

if __name__ == '__main__':
    print(*python_y2025_d07())
