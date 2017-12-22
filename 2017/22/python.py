#!/usr/bin/env python3
from sys import stdin
infected = set()
for y, line in enumerate(stdin):
    x = line.find('#')
    while x != -1:
        infected.add(complex(x, y))
        x = line.find('#', x + 1)
pos = complex(len(line.strip()) // 2, (y + 1) // 2)
dir = -1j
ibursts = 0
for n in range(10000):
    dir *= 1j if pos in infected else -1j
    if pos not in infected:
        infected.add(pos)
        ibursts += 1
    else:
        infected.remove(pos)
    pos += dir
print(ibursts)
