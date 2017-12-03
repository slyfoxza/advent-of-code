#!/usr/bin/env python3
from sys import stdin
target = int(stdin.read())
r = 1
sq0 = 1
nsq = 8
while sq0 + nsq < target:
    r += 1
    sq0 += nsq
    nsq += 8
fsize = nsq // 4
face = (target - sq0 - 1) // fsize
foff = target - sq0 - 1 - face * fsize
if face == 0:
    x, y = r, -r+1 + foff
elif face == 1:
    x, y = r - foff - 1, r
elif face == 2:
    x, y = -r, r - foff - 1
else:
    x, y = -r + foff + 1, -r
dist = abs(x) + abs(y)

grid = {0: 1}
pos = 1
dir = 1j
last = 1
while last <= target:
    locs = [-1-1j, -1j, 1-1j,
               -1,      1,
            -1+1j,  1j, 1+1j]
    last = 0
    for loc in locs:
        neighbour = grid.get(pos + loc)
        if neighbour is None:
            continue
        last += neighbour
    grid[pos] = last
    pos += dir
    if pos.real > 0 and pos.real == -pos.imag:
        pass
    elif dir == 1 and pos.real == -pos.imag + 1:
        dir *= 1j
    elif abs(pos.real) == abs(pos.imag):
        dir *= 1j
print(abs(x) + abs(y), last)
