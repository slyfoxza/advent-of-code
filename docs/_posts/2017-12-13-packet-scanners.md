---
title: Packet Scanners
date: 2017-12-13 13:08:28 -0500
---
# D
This one was a throwback to [December 15, 2016][2016d15]! Spent some time
trying to figure out where in [Phobos][phobos] the "map" class was, only to
realise that [associative arrays are a first-class thing in D][dassoc].

Anyway, this was pretty easy when you realise that the period of each scanner
is `2n-1`, where _n_ is the range thereof.

I suppose the smart way to solve part 2 would've been an application of the
Chinese Remainder Theorem, as more mathematically-inclined people pointed out
in [last year's solution megathread][2016d15mt], but screw it: [here we see the
common brute force loop in its natural habitat, consuming a rich diet of CPU
cycles][1]. This, by the way, also shows the glory of labelled loop control
statements. Bless 'em.

[1]: https://github.com/slyfoxza/advent-of-code/blob/11beb19721c0ab59f56f14fa6be193498cf25f04/2017/13/d.d
[2016d15]: http://adventofcode.com/2016/day/15
[2016d15mt]: https://www.reddit.com/r/adventofcode/comments/5ifn4v/2016_day_15_solutions/
[dassoc]: https://dlang.org/spec/hash-map.html
[phobos]: https://dlang.org/phobos/index.html
