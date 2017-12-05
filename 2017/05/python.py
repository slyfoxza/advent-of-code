#!/usr/bin/env python3
from sys import stdin

offsets1 = [int(e) for e in stdin.read().split()]
offsets2 = list(offsets1)
length = len(offsets1)
n = [0, 0]

i = 0
while 0 <= i < length:
    offsets1[i] += 1
    i += offsets1[i] - 1
    n[0] += 1

i = 0
while 0 <= i < length:
    offset = offsets2[i]
    offsets2[i] += (1 if offset < 3 else -1)
    i += offset
    n[1] += 1
print(*n)
