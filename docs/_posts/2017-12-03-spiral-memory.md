---
title: Spiral Memory
date: 2017-12-03 01:39:36 -0500
---
# Python
A _slight_ increase in difficulty, compared to the previous days.

[Part one][python1] uses a more-or-less-constant-time determination of the
Cartesian coordinates of the given square index, taking advantage of the fact
that each winding of the spiral contains 8 squares more than the previous
winding, starting with 8.  Using this, the radius `r` of the winding the target
square is in, as well as its location within that winding in terms of square
faces can be calculated.

Determining the Cartesian coordinates is then a simple offset using the radius
value, depending on which of the four faces the square falls in. The [Manhattan
distance][manhattan] follows simply from that, and then part one is done!

For [part two][python2] I couldn't think of a better way to approach the
problem than a traversal of the spiral until the target condition was
satisfied. For storage, I used a [dictionary][pydict] keyed by a [complex
number][pynum] representing a Cartesian coordinate; the real component
represents the _x_-axis, while the imaginary component represents the _y_-axis.

Using complex numbers has the useful property that rotating the direction of
travel to the left is a simple multiplication of itself by the imaginary unit,
while advancing a position along that direction is a simple addition thereof,
getting rid of what would otherwise be a tedious 4-branched conditional block.

[manhattan]: https://en.wikipedia.org/wiki/Taxicab_geometry
[pydict]: https://docs.python.org/3/library/stdtypes.html#mapping-types-dict
[pynum]: https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex
[python1]: https://github.com/slyfoxza/advent-of-code/blob/67e6b7fbb53cc0cc5ddf8fb6c49b8b545d0bb214/2017/03/python.py#L4-L22
[python2]: https://github.com/slyfoxza/advent-of-code/blob/67e6b7fbb53cc0cc5ddf8fb6c49b8b545d0bb214/2017/03/python.py#L24-L45
