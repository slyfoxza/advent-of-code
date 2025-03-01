---
title: Inverse Captcha
date: 2017-12-01 01:14:27 -0500
---
# Ruby
For the first part, I merely (ab)used the [`String#scan`][1] method, using a
[regular expression][ruby1] to accumulate repeated digits, initialising the
accumulator based on the first and last digits.

This solution fell apart when the second half reared its head, so I [changed
tack][ruby2] to use character indexing instead. To ease the burden of the
circular string, I monkey-patched [`String#[]`][2] to integrate the use of the
modulus operator.

# Python
Nothing too fancy. A [generator expression][python] takes care of converting
digit characters to integer values, while `enumerate` is used to obtain the
index of each digit in the string.

# Rust
[Even less fancy than the Python implementation.][rust]

# Java
An [entirely unnecessary usage][java] of the Java 9 module system.

[1]: http://ruby-doc.org/core-2.4.2/String.html#method-i-scan
[2]: http://ruby-doc.org/core-2.4.2/String.html#method-i-5B-5D
[java]: https://github.com/slyfoxza/advent-of-code/blob/b8154b763095474dacfd2acadf9aca5c7f91bc76/2017/01/java-source/aoc/AdventOfCode.java
[python]: https://github.com/slyfoxza/advent-of-code/blob/7d7b96135c981dc9526fc8d5c24180eb8d020042/2017/01/python.py#L5
[ruby1]: https://github.com/slyfoxza/advent-of-code/blob/a5d27993fd3ba4602bd7478b68349e7f89ddb4dd/2017/01/test1.rb#L4
[ruby2]: https://github.com/slyfoxza/advent-of-code/blob/094c5a21c2b56f0d4c1de95f5f2ce959c40ed411/2017/01/test1.rb
[rust]: https://github.com/slyfoxza/advent-of-code/blob/a0cf1ef301a09fc3d9c14ef161a0455f35cbf95f/2017/01/rust.rs
