---
title: Corruption Checksum
date: 2017-12-02 00:50:30 -0500
---
# Python
[The power of `itertools` compels thee!][itertools] `combinations` is used to
iterate pair-wise over the cells in a row, avoiding visiting a given pair
twice.

Since division is not commutative, `permutations` is used to [reduce what would
otherwise be a tedious and verbose conditional statement to swap the operand
order][solution].

[itertools]: https://docs.python.org/3/library/itertools.html
[solution]: https://github.com/slyfoxza/advent-of-code/blob/eabd1490eaffa0e60a9f31394d6d277078932cae/2017/02/python.py#L13-L17
