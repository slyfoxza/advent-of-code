#!/usr/bin/env ruby
$program = $stdin.each_line.map { |x| x.split }
$regs = {'a' => 1}
$n_mul = 0
def run(max_ip=$program.length)
  ip = 0
  until ip < 0 || ip >= max_ip
    ins, x, y = $program[ip]
    y = $regs[y] if y =~ /[a-z]/
    y = y.to_i

    case ins
    when 'set'; $regs[x] = y
    when 'sub'; $regs[x] -= y
    when 'mul'
      $regs[x] *= y
      $n_mul += 1
    when 'jnz'
      x = $regs[x] if x =~ /[a-z]/
      x = x.to_i
      if x != 0
        ip += y
        next
      end
    end

    ip += 1
  end
end
run(8)
b, c = $regs['b'], $regs['c']
$regs = {}
$n_mul = 0
run
h = 0
(b..c).step(17).each do |b|
  if (b & 1) == 0 || b % 3 == 0
    h += 1
    next
  end
  catch :factored do
    i = 5
    while i * i < b
      if b % i == 0 || b % (i + 2) == 0
        h += 1
        throw :factored
      end
      i += 6
    end
  end
end
puts "#{$n_mul} #{h}"
