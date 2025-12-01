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
def python_y2015_d14():
    all_reindeer = dict()
    with open("y2015/d14/input") as f:
        while len(line := f.readline()) != 0:
            bits = line.split()
            all_reindeer[bits[0]] = {
                'distance': 0,
                'score': 0,
                'speed': int(bits[3]),
                'speed_dur': int(bits[6]),
                'rest_dur': int(bits[-2]),
                'is_rest': False,
            }
            all_reindeer[bits[0]]['tick'] = all_reindeer[bits[0]]['speed_dur']

    for i in range(2503):
        for reindeer in all_reindeer.values():
            reindeer['tick'] -= 1
            if reindeer['is_rest'] and reindeer['tick'] == 0:
                reindeer['is_rest'] = False
                reindeer['tick'] = reindeer['speed_dur']
            elif not reindeer['is_rest']:
                reindeer['distance'] += reindeer['speed']
                if reindeer['tick'] == 0:
                    reindeer['is_rest'] = True
                    reindeer['tick'] = reindeer['rest_dur']

        max_dist = max(d['distance'] for d in all_reindeer.values())
        for reindeer in all_reindeer.values():
            if reindeer['distance'] == max_dist:
                reindeer['score'] += 1

    return (
        max(d['distance'] for d in all_reindeer.values()),
        max(d['score'] for d in all_reindeer.values()),
    )

if __name__ == "__main__":
    print(*python_y2015_d13())
