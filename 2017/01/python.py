#!/usr/bin/env python3
from sys import stdin

accum = [0, 0]
input = [int(c) for c in stdin.read().strip()]
len = len(input)
offset = (1, int(len / 2))
for i, c in enumerate(input):
    for j, o in enumerate(offset):
        if input[(i + o) % len] == c:
            accum[j] += c
print(*accum)
