#!/usr/bin/env ruby
@input = $stdin.read.strip.chars

def parse(i, s0)
  g = 0
  s = 0
  while i < @input.length
    case @input[i]
    when '{'
      s += s0
      i, ss, gs = parse i+1, s0+1
      g += gs
      s += ss
    when '<'
      i += 1
      while i < @input.length
        if @input[i] == '>'
          break
        elsif @input[i] == '!'
          i += 2
        else
          g += 1
          i += 1
        end
      end
    when '}'
      return i+1, s+1, g
    else
      i += 1
    end
  end
  [i, s, g]
end

r = parse 0, 0
puts r[1..2].join(' ')
