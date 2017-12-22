#!/usr/bin/env python3
from sys import stdin
infected = set()
for y, line in enumerate(stdin):
    x = line.find('#')
    while x != -1:
        infected.add(complex(x, y))
        x = line.find('#', x + 1)
infected_copy = set(infected)
ibursts = [0, 0]
pos = complex(len(line.strip()) // 2, (y + 1) // 2)
dir = -1j
for n in range(10000):
    dir *= 1j if pos in infected else -1j
    if pos not in infected:
        infected.add(pos)
        ibursts[0] += 1
    else:
        infected.remove(pos)
    pos += dir

infected = infected_copy
weakened = set()
flagged = set()
pos = complex(len(line.strip()) // 2, (y + 1) // 2)
dir = -1j
for n in range(10000000):
    if pos in infected:
        dir *= 1j
        infected.remove(pos)
        flagged.add(pos)
    elif pos in flagged:
        dir *= -1
        flagged.remove(pos)
    elif pos not in weakened:
        dir *= -1j
        weakened.add(pos)
    else:
        weakened.remove(pos)
        infected.add(pos)
        ibursts[1] += 1
    pos += dir

print(ibursts)
