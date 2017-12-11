#!/usr/bin/env ruby
def D(q, r); [q, r, -q-r].map(&:abs).max; end
q, r = 0, 0
m = 0
$stdin.read.strip.split(',').each do |d|
  case d
  when 'n'
    r -= 1
  when 'ne'
    q += 1
    r -= 1
  when 'se'
    q += 1
  when 's'
    r += 1
  when 'sw'
    q -= 1
    r += 1
  when 'nw'
    q -= 1
  end
  m = [m, D(q, r)].max
end
puts "#{D(q, r)} #{m}"
