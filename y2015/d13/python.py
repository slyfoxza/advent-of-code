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
from itertools import permutations

def python_y2015_d13():
    people = dict()
    with open("y2015/d13/input") as f:
        while len(line := f.readline()) != 0:
            bits = line.split()
            person = bits[0]
            sign = 1 if bits[2] == 'gain' else -1
            other = bits[-1][:-1] # strip trailing period
            if person not in people:
                people[person] = dict()
            people[person][other] = sign * int(bits[3])

    max_happiness = 0
    for arrangement in permutations(people.keys()):
        happiness = 0
        for i in range(len(arrangement)):
            person = people[arrangement[i]]
            happiness += person[arrangement[(i - 1) % len(arrangement)]]
            happiness += person[arrangement[(i + 1) % len(arrangement)]]
        max_happiness = max(happiness, max_happiness)

    people["Me"] = dict()
    max_happiness_me = 0
    for arrangement in permutations(people.keys()):
        happiness = 0
        for i in range(len(arrangement)):
            person = people[arrangement[i]]
            happiness += person.get(arrangement[(i - 1) % len(arrangement)], 0)
            happiness += person.get(arrangement[(i + 1) % len(arrangement)], 0)
        max_happiness_me = max(happiness, max_happiness_me)

    return (max_happiness, max_happiness_me)

if __name__ == "__main__":
    print(*python_y2015_d13())
