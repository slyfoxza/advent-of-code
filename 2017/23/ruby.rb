#!/usr/bin/env ruby
program = $stdin.each_line.map { |x| x.split }
regs = {}
n_mul = 0
ip = 0
until ip < 0 || ip >= program.length
  ins, x, y = program[ip]
  y = regs[y] if y =~ /[a-z]/
  y = y.to_i

  case ins
  when 'set'; regs[x] = y
  when 'sub'; regs[x] -= y
  when 'mul'
    regs[x] *= y
    n_mul += 1
  when 'jnz'
    x = regs[x] if x =~ /[a-z]/
    x = x.to_i
    if x != 0
      ip += y
      next
    end
  end

  ip += 1
end
puts n_mul
