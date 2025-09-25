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
from hashlib import md5

def python_y2015_d04():
    part1 = 0

    with open("y2015/d04/input", "rb") as f:
        secret_key = f.read().rstrip()

    i = 1
    while True:
        h = md5()
        h.update(secret_key)
        h.update(str(i).encode())
        h = h.digest()

        if h.startswith(b"\x00\x00"):
            if part1 == 0 and h[2] & 0xF0 == 0:
                part1 = i
            elif h[2] == 0:
                return (part1, i)

        i += 1

if __name__ == "__main__":
    print(*python_y2015_d04())
