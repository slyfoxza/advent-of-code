#!/usr/bin/env python3
from sys import stdin
L = 256
i, s, v = 0, 0, list(range(0, L))
for x in [int(e) for e in stdin.read().strip().split(',')]:
    if x == 1:
        pass
    elif i + x < L:
        v[i:i+x] = reversed(v[i:i+x])
    else:
        r = list(reversed(v[i:] + v[:i+x-L]))
        v[i:] = r[:L-i]
        v[:i+x-L] = r[L-i-x:]
    i = (i + x + s) % L
    s += 1
print(v[0] * v[1])
