#!/usr/bin/env python3
from functools import reduce
from sys import stdin
KEY = stdin.read().strip()
L = 256 # Sparse hash length
def rev(i, s, x, v):
    if x == 1:
        pass
    elif i + x < L:
        v[i:i+x] = reversed(v[i:i+x])
    else:
        j = i + x - L
        r = list(reversed(v[i:] + v[:j]))
        v[i:] = r[:L-i]
        v[:j] = r[L-i:]
    return (i + x + s) % L, s + 1
grid = []
ones = 0
for i in range(128):
    hashin = KEY + '-' + str(i)
    j, skip, sparse = 0, 0, list(range(L))
    hashin = [ord(e) for e in hashin] + [17, 31, 73, 47, 23]
    for _ in range(64):
        for x in hashin:
            j, skip = rev(j, skip, x, sparse)
    d = [reduce(lambda x,y: x^y, sparse[x*16:(x+1)*16]) for x in range(16)]
    d = ''.join(format(e, '08b') for e in d)
    row = [int(x) for x in d]
    grid.append(row)
    ones += sum(row)
region = 0
for x in range(128):
    for y in range(128):
        if grid[x][y] == 0:
            continue
        adj = [(x, y)]
        while len(adj) != 0:
            i, j = adj.pop()
            grid[i][j] = 0
            if i > 0 and grid[i - 1][j] == 1:
                adj.append((i - 1, j))
            if j > 0 and grid[i][j - 1] == 1:
                adj.append((i, j - 1))
            if i < 127 and grid[i + 1][j] == 1:
                adj.append((i + 1, j))
            if j < 127 and grid[i][j + 1] == 1:
                adj.append((i, j + 1))
        region += 1
print(ones, region)
