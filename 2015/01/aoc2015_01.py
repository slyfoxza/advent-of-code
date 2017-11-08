#!/usr/bin/env python
import sys

def chars(io):
    while True:
        c = io.read(1)
        if not c: break
        yield c

def delta(c): return {'(': 1, ')': -1}.get(c, 0)

accum = 0
negind = None
for i, d in enumerate(map(delta, chars(sys.stdin))):
    accum = accum + d
    if accum < 0 and negind is None: negind = i + 1
print(accum, negind)
