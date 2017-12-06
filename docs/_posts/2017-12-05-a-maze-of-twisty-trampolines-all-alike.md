---
title: A Maze of Twisty Trampolines, All Alike
date: 2017-12-05 00:58:05 -0500
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

# Ruby
My [Ruby solution][ruby] fares a bit better than Python (given the similar
implementation) clocking in at just shy of 10 seconds; still pretty slow
compared to Node.js.

# Rust
Now that we've got a decent spread in terms of representation from the
interpreted/script languages, let's give the compiled binaries a crack at it.

[Last year's darling][aoc2016], [Rust][rust], rises to the task, completing it
in 90 milliseconds or so.

# C
Interestingly, the [C][c] solution is 10-20 ms slower than Rust. I suspect this
may be due to the repeated calls to `scanf` when reading the input.

# C++
I didn't necessarily write the [C++ solution][cpp] for speed; rather, I abused
function templates and lambdas to remove the repetition of the loops' boundary
conditions and iteration counting.

For what it's worth, this implementation was slower than the C implementation
by another 20 ms.

[1]: https://i.imgur.com/ASvCFmOh.jpg
[aoc2016]: https://github.com/slyfoxza/advent-of-code/tree/eccc9f9113f00fe3ab84ebf59f38df1685fef3db/2016
[c]: https://github.com/slyfoxza/advent-of-code/blob/6a45b4f02006cfddb319b33b38d61a1c966b9b92/2017/05/c.c
[cpp]: https://github.com/slyfoxza/advent-of-code/blob/0c29fffbfafcf70ef36cf6dc0dd833f8bd43c5cb/2017/05/c%2B%2B.c%2B%2B
[cprofile]: https://docs.python.org/3/library/profile.html
[helloserve]: https://github.com/helloserve/adventofcode/blob/master/2017/day5.js
[nodejs]: https://github.com/slyfoxza/advent-of-code/blob/2ff7ce14f116b93e9c2113ff170b1a65700653a6/2017/05/node.js
[pycondex]: https://docs.python.org/3/reference/expressions.html#conditional-expressions
[pyopchain]: https://docs.python.org/3/reference/expressions.html#comparisons
[python1]: https://github.com/slyfoxza/advent-of-code/blob/c9dbe4e51261e8ed6a9502c54453f68928ad6f77/2017/05/python.py
[python2]: https://github.com/slyfoxza/advent-of-code/blob/c9dbe4e51261e8ed6a9502c54453f68928ad6f77/2017/05/python.py#L17
[python3]: https://github.com/slyfoxza/advent-of-code/blob/a19f317ab634e53135e96789e25f4ebef8136a31/2017/05/python.py
[ruby]: https://github.com/slyfoxza/advent-of-code/blob/c8c6a615666b464e96a96d416d2070665024767e/2017/05/ruby.rb
[rust]: https://github.com/slyfoxza/advent-of-code/blob/0bc2f610e3661dbebc100796a1fd14f73390849a/2017/05/rust.rs
