#!/usr/bin/env python3
import itertools
import sys

accum1 = 0
accum2 = 0
for row in sys.stdin.readlines():
    mini = maxi = None
    for pair in itertools.combinations((int(cell) for cell in row.split()), 2):
        mini = pair[0] if mini is None else min(mini, pair[0])
        maxi = pair[0] if maxi is None else max(maxi, pair[0])

        for lhs, rhs in itertools.permutations(pair):
            q, r = divmod(lhs, rhs)
            if r == 0:
                accum2 += q
                break
    accum1 += maxi - mini
print(accum1, accum2)
