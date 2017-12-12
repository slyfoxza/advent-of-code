---
title: Digital Plumber
date: 2017-12-12 13:29:28 -0500
---
# Ruby
Another day, another [Ruby solution][1]. The general idea (_salute emoji_) is
to [populate the `P` array][2] using the source program as index, and an array
of connected programs as value; basically, translating the input text to that
data structure.

After that, [the `G` array is systematically populated][3] with unique groups:
the index is the group identifier, and the value is an array of all programs in
that group. This is done by looping over each program, and attempting to reach
all (indirectly) connected programs. If an index is already contained in a
group that has been completely populated, the iteration is skipped.

Once that loop is complete, getting the answers for both parts is as simple as
reading the length of group 0, and then the length of the groups array.

[1]: https://github.com/slyfoxza/advent-of-code/blob/d364708faf87ec9c8a20fa5d60bb0431ebd607bf/2017/12/ruby.rb
[2]: https://github.com/slyfoxza/advent-of-code/blob/d364708faf87ec9c8a20fa5d60bb0431ebd607bf/2017/12/ruby.rb#L4-L8
[3]: https://github.com/slyfoxza/advent-of-code/blob/d364708faf87ec9c8a20fa5d60bb0431ebd607bf/2017/12/ruby.rb#L9-L21
