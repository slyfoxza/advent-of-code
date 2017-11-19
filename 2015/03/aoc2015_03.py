#!/usr/bin/env python
import sys

x = y = 0
uniq = set(((x, y),))
for c in sys.stdin.read():
    if c == '<': x -= 1
    elif c == '>': x += 1
    elif c == '^': y += 1
    elif c == 'v': y -= 1
    uniq.add((x, y))
print(len(uniq))
