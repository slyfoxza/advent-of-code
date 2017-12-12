#!/usr/bin/env ruby
require 'set'
P = [[0]]
$stdin.read.strip.lines.map { |x| x.strip.split(' <-> ', 2) }.each do |l|
  i = l[0].to_i
  P.fill [], P.length, i - P.length + 1 if i > P.length
  P[i] = (P[i] || []) + l[1].split(', ').map(&:to_i)
end
S = Set.new
C = Array.new P[0]
c = 0
until C.empty?
  n = C.pop
  next if S.include? n
  S << n
  C.concat P[n]
  c += 1
end
puts c
