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
def python_y2025_d02():
    with open("y2025/d02/input") as f:
        finput = f.read().rstrip()

    id_sum = 0
    id_sum2 = 0
    for id_range in finput.split(','):
        start, end = list(int(x) for x in id_range.split('-'))
        for i in range(int(start), int(end) + 1):
            s = str(i)

            mid = len(s) // 2
            if s[:mid] == s[mid:]:
                id_sum += i

            for n in range(1, mid + 1):
                if len(s) % n != 0:
                    continue
                sub = s[:n]
                for j in range(n, len(s), n):
                    if s[j:j+n] != sub:
                        break
                else:
                    id_sum2 += i
                    break

    return (id_sum, id_sum2)

if __name__ == '__main__':
    print(*python_y2025_d02())
