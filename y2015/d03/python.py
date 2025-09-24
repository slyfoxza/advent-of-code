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
from copy import copy
from dataclasses import dataclass

@dataclass(unsafe_hash=True)
class Point:
    x: int
    y: int

def python_y2015_d03():
    visited1 = set()
    visited2 = set()

    solo_santa_pos = Point(0, 0)
    visited1.add(copy(solo_santa_pos))

    santa_pos = Point(0, 0)
    robo_pos = Point(0, 0)
    current = robo_pos
    visited2.add(copy(santa_pos))

    with open("y2015/d03/input") as f:
        while len(buffer := f.read()) != 0:
            for c in buffer:
                current = santa_pos if current is robo_pos else robo_pos

                if c == '^':
                    solo_santa_pos.y -= 1
                    current.y -= 1
                elif c == 'v':
                    solo_santa_pos.y += 1
                    current.y += 1
                elif c == '<':
                    solo_santa_pos.x -= 1
                    current.x -= 1
                elif c == '>':
                    solo_santa_pos.x += 1
                    current.x += 1

                visited1.add(copy(solo_santa_pos))
                visited2.add(copy(current))
    return (len(visited1), len(visited2))

if __name__ == "__main__":
    print(*python_y2015_d03())
