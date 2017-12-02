#!/usr/bin/env ruby
class String
  old = instance_method(:[])
  define_method(:[]) do |i|
    old.bind(self).(i % length)
  end
end
input = $stdin.read.strip
accum = [0, 0]
half = input.length / 2
input.each_char.with_index do |c, i|
  accum[0] += c.to_i if input[i + 1] == c
  accum[1] += c.to_i if input[i + half] == c
end
puts accum.join(' ')
