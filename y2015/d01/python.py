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
def python_y2015_d01():
    with open("y2015/d01/input") as f:
        floor = 0
        position = -1
        i = 0
        while len(buffer := f.read()) != 0:
            for c in buffer:
                if c == '(':
                    floor += 1
                elif c == ')':
                    floor -= 1

                i += 1
                if position == -1 and floor < 0:
                    position = i
    return (floor, position)

if __name__ == "__main__":
    print(*python_y2015_d01())
