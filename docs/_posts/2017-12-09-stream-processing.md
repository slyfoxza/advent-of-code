---
title: Stream Processing
date: 2017-12-09 00:47:33 -0500
---
# Ruby
I decided to [solve today's challenge][ruby2] using recursion and a moving
index through the input characters. Some of this decision was probably driven
by coding defensively at what the second part would've been, which turns out to
have been unnecessary.

For one, I'd probably be able to treat the input as a stream, rather than
buffering the input in memory. The recursion is probably a mild form of
overkill, too.

[ruby2]: https://github.com/slyfoxza/advent-of-code/blob/ec31928f7b0fcfdb6b3cdfa9a788f82b00918057/2017/09/ruby.rb
