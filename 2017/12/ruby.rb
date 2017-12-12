#!/usr/bin/env ruby
require 'set'
P, G = [[0]], []
$stdin.read.strip.lines.map { |x| x.strip.split(' <-> ', 2) }.each do |l|
  i = l[0].to_i
  P.fill [], P.length, i - P.length + 1 if i > P.length
  P[i] = (P[i] || []) + l[1].split(', ').map(&:to_i)
end
g = 0
(0...P.length).each do |i|
  next if G[0, g].any? { |x| x.include? i }
  s, c = Set.new, Array.new(P[i])
  until c.empty?
    n = c.pop
    next if s.include? n
    s << n
    G[g] = (G[g] || []) << n
    c.concat P[n]
  end
  g += 1
end
puts "#{G[0].length} #{G.length}"
