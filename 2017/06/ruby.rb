#!/usr/bin/env ruby
state = $stdin.read.strip.split.map &:to_i
seen = []
n = 0
until seen.include? state.join ' '
  seen << state.join(' ')
  i = state.find_index state.max
  val = state[i]
  state[i] = 0
  until val == 0
    i += 1
    state[i % 16] += 1
    val -= 1
  end
  n += 1
end
puts "#{n} #{seen.length - seen.find_index(state.join ' ')}"
