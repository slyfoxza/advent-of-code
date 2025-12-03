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
def joltage(bank, n=2):
    s = ''
    i = 0
    while n > 0:
        banksub = bank[i:1-n or None]
        digit = max(banksub)
        s += digit
        i = bank.index(digit, i) + 1
        n -= 1
    return int(s)

def python_y2025_d03():
    joltage_sum = 0
    joltage_12sum = 0
    with open("y2025/d03/input") as f:
        while len(line := f.readline().rstrip()) != 0:
            joltage_sum += joltage(line)
            joltage_12sum += joltage(line, 12)
    return (joltage_sum, joltage_12sum)

if __name__ == '__main__':
    print(*python_y2025_d03())
