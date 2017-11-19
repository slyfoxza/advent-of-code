#!/usr/bin/env python
import sys

locs = ([0, 0], [0, 0], [0, 0])
loci = 0
solo = set(((0, 0),))
robo = set(((0, 0),))

def go(c, loc, uniq):
    if c == '<': loc[0] -= 1
    elif c == '>': loc[0] += 1
    elif c == '^': loc[1] += 1
    elif c == 'v': loc[1] -= 1
    else: return
    uniq.add((loc[0], loc[1]))

for c in sys.stdin.read():
    go(c, locs[2], solo)
    go(c, locs[loci], robo)
    loci = (loci + 1) & 1

print(len(solo), len(robo))
