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
def python_y2025_d05():
    fresh_count = 0
    fresh_range_count = 0
    fresh_ranges = []
    with open("y2025/d05/input") as f:
        while len(line := f.readline().rstrip()) != 0:
            s, e = (int(x) for x in line.split('-'))
            new_range = range(s, e + 1)
            while new_range is not None:
                s, e = new_range.start, new_range.stop - 1
                for i in range(len(fresh_ranges)):
                    existing = fresh_ranges[i]
                    if s >= existing.start and e < existing.stop:
                        new_range = None
                        break
                    elif existing.start >= s and existing.stop <= e:
                        del fresh_ranges[i]
                        break

                    extends_start = s < existing.start and existing.start <= e < existing.stop
                    extends_end = existing.start <= s < existing.stop and e >= existing.stop
                    if extends_start:
                        del fresh_ranges[i]
                        new_range = range(s, existing.stop)
                        break
                    elif extends_end:
                        del fresh_ranges[i]
                        new_range = range(existing.start, e + 1)
                        break
                else:
                    fresh_ranges.append(new_range)
                    new_range = None

        for fresh_range in fresh_ranges:
            width = fresh_range.stop - fresh_range.start
            fresh_range_count += width

        while len(line := f.readline().rstrip()) != 0:
            id = int(line)
            for fresh_range in fresh_ranges:
                if id in fresh_range:
                    fresh_count += 1
                    break

    return (fresh_count, fresh_range_count)

if __name__ == '__main__':
    print(*python_y2025_d05())
