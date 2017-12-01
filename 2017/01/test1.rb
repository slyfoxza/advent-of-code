#!/usr/bin/env ruby
input = $stdin.read.strip
accum = input[0] == input[input.length - 1] ? input[0].to_i : 0
input.scan(/(\d)(?=\1)/) { |d| accum += d[0].to_i }
puts accum
