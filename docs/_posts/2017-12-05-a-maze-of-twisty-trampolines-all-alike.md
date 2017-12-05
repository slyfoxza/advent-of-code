---
title: A Maze of Twisty Trampolines, All Alike
date: 2017-12-05 00:58:05 -0200
---
# Python
I [picked Python again][python1] because I could leverage [operator
chaining][pyopchain] in the loop conditional expression. A [conditional
expression][pycondex] also features in the [second part][python2].

# Node.js
The Python solution runs… slowly; roughly 20 seconds on my laptop. When
[Henk][helloserve] told me his Node.js solution ran in 600 ms, I was like
![¿que?][1]{:class='mfw'}

Turns out, it's true! My [Node.js implementation][nodejs] runs in roughly 320
ms, which means Python gets to be about 2 orders of magnitude slower.

Enter: [the `cProfile` module][cprofile]. Turns out, at least 2-3 seconds of
the Python run time was being spent repeatedly calling `len` to evaluate the
loop condition (output abbreviated):

      ncalls  tottime  percall  cumtime  percall filename:lineno(function)
           1   17.400   17.400   19.571   19.571 python.py:2(<module>)
    30905221    2.171    0.000    2.171    0.000 {built-in method builtins.len}

[Refactoring to cache the array length][python3] resulted in a noticeable
improvement (again, abbreviated):

      ncalls  tottime  percall  cumtime  percall filename:lineno(function)
           1   13.036   13.036   13.036   13.036 python.py:2(<module>)

While this is better, it's still pretty bad, but it seems that this is just the
cost of doing business in Python.

[1]: https://i.imgur.com/ASvCFmOh.jpg
[cprofile]: https://docs.python.org/3/library/profile.html
[helloserve]: https://github.com/helloserve/adventofcode/blob/master/2017/day5.js
[nodejs]: https://github.com/slyfoxza/advent-of-code/blob/2ff7ce14f116b93e9c2113ff170b1a65700653a6/2017/05/node.js
[pycondex]: https://docs.python.org/3/reference/expressions.html#conditional-expressions
[pyopchain]: https://docs.python.org/3/reference/expressions.html#comparisons
[python1]: https://github.com/slyfoxza/advent-of-code/blob/c9dbe4e51261e8ed6a9502c54453f68928ad6f77/2017/05/python.py
[python2]: https://github.com/slyfoxza/advent-of-code/blob/c9dbe4e51261e8ed6a9502c54453f68928ad6f77/2017/05/python.py#L17
[python3]: https://github.com/slyfoxza/advent-of-code/blob/a19f317ab634e53135e96789e25f4ebef8136a31/2017/05/python.py
