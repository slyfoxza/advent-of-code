#!/usr/bin/env ruby
ORDER = ('a'..'p').to_a
LEN = ORDER.length
$stdin.read.strip.split(',').each do |move|
  if move[0] == ?s
    length = move[1..-1].to_i
    head, tail = ORDER[0...-length], ORDER[-length..-1]
    ORDER[0...length] = tail
    ORDER[length..-1] = head
  elsif move =~ %r[([px])(\w+)/(\w+)]
    if $~[1] == ?p
      a, b = [2, 3].map { |x| ORDER.index $~[x] }
    elsif $~[1] == ?x
      a, b = [2, 3].map { |x| $~[x].to_i }
    end
    ORDER[a], ORDER[b] = ORDER[b], ORDER[a]
  end
end
puts ORDER.join
