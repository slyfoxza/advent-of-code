#!/usr/bin/env python3
import re
from sys import stdin
blueprint = stdin.read()
init_state, n_steps = re.search(r'(?m)\ABegin in state ([A-Z])\.\nPerform a diagnostic checksum after (\d+) steps.', blueprint).group(1, 2)
regex = re.compile(r'(?m)In state ([A-Z]):((?:\n  .+)+)')#\n.+(\d+):\n.+(\d+)\.\n.+(left|right)\.\n.+([A-Z])\.')
re_in = re.compile(r'(?m)If the current value is (\d+):\n.+(\d+)\.\n.+(left|right)\.\n.+([A-Z])\.')
states = {}
for state, state_bp in regex.findall(blueprint):
    states[state] = [None, None]
    for cv, wv, mv, ns in re_in.findall(state_bp):
        states[state][int(cv)] = (int(wv), -1 if mv == 'left' else 1, ns)
i = 0
tape = [0]
state = init_state
for n in range(int(n_steps)):
    cv = tape[i]
    tape[i] = states[state][cv][0]
    i += states[state][cv][1]
    if i < 0:
        tape.insert(0, 0)
        i += 1
    elif i >= len(tape):
        tape.append(0)
    state = states[state][cv][2]
print(sum(tape))
