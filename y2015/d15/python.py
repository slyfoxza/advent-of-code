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
import itertools

def python_y2015_d15():
    ingredients = []
    with open("y2015/d15/input") as f:
        while len(line := f.readline().rstrip()) != 0:
            name, line = line.split(':')
            ing_props = []
            for ingredient in line.split(','):
                ing_prop, value = ingredient.split()
                value = int(value)
                ing_props.append(value)
            ingredients.append(ing_props)

    max_score1 = 0
    max_score2 = 0
    for teaspoons in itertools.product(*([range(0, 101)] * len(ingredients))):
        if sum(teaspoons) > 100:
            continue

        scores = [0] * (len(ingredients[0]))
        for i in range(len(ingredients)):
            for j in range(len(ingredients[i])):
                scores[j] += ingredients[i][j] * teaspoons[i]

        total_score1 = scores[0]
        total_score2 = scores[0]
        for i in range(1, len(scores) - 1):
            total_score1 *= max(0, scores[i])
            total_score2 *= max(0, scores[i])

        max_score1 = max(total_score1, max_score1)
        if scores[-1] == 500:
            max_score2 = max(total_score2, max_score2)

    return (max_score1, max_score2)

if __name__ == '__main__':
    print(*python_y2015_d15())
