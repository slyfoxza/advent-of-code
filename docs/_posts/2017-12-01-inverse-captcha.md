---
title: Inverse Captcha
date: 2017-12-01 01:14:27 -0500
---
# Ruby
For the first part, I merely (ab)used the [`String#scan`][1] method, using a
[regular expression][p1] to accumulate repeated digits, initialising the
accumulator based on the first and last digits.

This solution fell apart when the second half reared its head, so I [changed
tack][p2] to use character indexing instead. To ease the burden of the circular
string, I monkey-patched [`String#[]`][2] to integrate the use of the modulus
operator.

[1]: http://ruby-doc.org/core-2.4.2/String.html#method-i-scan
[2]: http://ruby-doc.org/core-2.4.2/String.html#method-i-5B-5D
[p1]: https://github.com/slyfoxza/advent-of-code/blob/a5d27993fd3ba4602bd7478b68349e7f89ddb4dd/2017/01/test1.rb#L4
[p2]: https://github.com/slyfoxza/advent-of-code/blob/094c5a21c2b56f0d4c1de95f5f2ce959c40ed411/2017/01/test1.rb
