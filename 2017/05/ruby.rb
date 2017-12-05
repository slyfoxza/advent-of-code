#!/usr/bin/env ruby
offsets = $stdin.read.strip.split.map &:to_i
offsets_copy = offsets.clone

steps = [0, 0]
i = 0
while (0...offsets.length).include? i
  offsets[i] += 1
  i += offsets[i] - 1
  steps[0] += 1
end

offsets = offsets_copy
i = 0
while (0...offsets.length).include? i
  offset = offsets[i]
  offsets[i] += offsets[i] >= 3 ? -1 : 1
  i += offset
  steps[1] += 1
end

puts steps.join(' ')
