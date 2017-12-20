#!/usr/bin/env python3
from sys import stdin
grid = [line for line in stdin]
pos = grid[0].index('|')
face = 1j
output = ''
steps = 0
while True:
    pos += face
    steps += 1
    y, x = int(pos.imag), int(pos.real)
    value = grid[y][x]
    if value == '+':
        next_pos = pos + (face * 1j)
        if grid[int(next_pos.imag)][int(next_pos.real)] == ' ':
            face *= -1j
        else:
            face *= 1j
    elif value == ' ':
        break
    elif value == '-' or value == '|':
        pass
    else:
        output += value
print(output, steps)
