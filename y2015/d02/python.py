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
def python_y2015_d02():
    paper = ribbon = 0
    with open("y2015/d02/input") as f:
        while len(line := f.readline()) != 0:
            w, h, l = map(int, line.split("x", 2))
            lw, wh, hl = l * w, w * h, h * l
            paper += 2 * (lw + wh + hl) + min(lw, wh, hl)
            ribbon += 2 * (l + w + h - max(l, w, h)) + l * w * h
    return (paper, ribbon)

if __name__ == "__main__":
    print(*python_y2015_d02())
