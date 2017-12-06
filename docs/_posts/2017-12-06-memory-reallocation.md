---
title: Memory Reallocation
date: 2017-12-06 00:24:32 -0500
---
# Ruby
To track previously-seen states, I simply maintained an array of strings
representing the 16 memory banks. On second thought, I could probably have
gotten away with just cloning the state array…

Anyway, the interior of the "until-seen" loop is pretty simple, finding the
index of the bank with the highest value, and then redistributing its wealth
one bank at a time. The redistribution loop can be done more efficiently, but
at 7 in the morning I was lazy. Basically, it should be possible to visit each
bank only once when the value to be redistributed is ≥ 16.

[The second part of the question][ruby2] is literally a one-line statement that
I embedded into the final output `puts`: because my state tracking data
structure is an array, the answer is trivially the distance between the array
end and the index of the element corresponding to the final (repeated) state.

[ruby2]: https://github.com/slyfoxza/advent-of-code/blob/11698b5352d02c0f01d90c88c5c207c357ff71a2/2017/06/ruby.rb
