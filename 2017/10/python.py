#!/usr/bin/env python3
from functools import reduce
from sys import stdin
L, F = 256, stdin.read().strip()
def do(i, s, x, v):
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
def a():
    i, s, v = 0, 0, list(range(L))
    for x in [int(e) for e in F.split(',')]:
        i, s = do(i, s, x, v)
    return v[0] * v[1]
def b():
    i, s, v, V = 0, 0, list(range(L)), [ord(e) for e in F] + [17, 31, 73, 47, 23]
    for n in range(64):
        for x in V:
            i, s = do(i, s, x, v)
    d = [reduce(lambda x,y: x^y, v[x*16:(x+1)*16]) for x in range(16)]
    return ''.join(format(e, '02x') for e in d)
print(a(), b())
