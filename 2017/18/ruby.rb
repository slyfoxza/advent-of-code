#!/usr/bin/env ruby
last_freq = nil
regs = {}
regs.default = 0
program = $stdin.each_line.map { |x| x.split }
i = 0
recover = nil
until i < 0 || i >= program.length
  ins, x, y = program[i]
  unless y.nil?
    y = regs[y.to_sym] if y =~ /[a-z]/
    y = y.to_i
  end
  #puts "#{ins} #{x} #{y}"

  case ins
  when 'snd'
    x = regs[x.to_sym] if x =~ /[a-z]/
    last_freq = x.to_i
  when 'rcv'
    x = regs[x.to_sym] if x =~ /[a-z]/
    if x.to_i != 0
      was_nil = recover == nil
      recover = last_freq
      if was_nil
        puts recover
        exit
      end
    end
  when 'set'; regs[x.to_sym] = y
  when 'add'; regs[x.to_sym] += y
  when 'mul'; regs[x.to_sym] *= y
  when 'mod'; regs[x.to_sym] %= y
  when 'jgz'
    x = regs[x.to_sym] if x =~ /[a-z]/
    if x.to_i > 0
      i += y
      next
    end
  end

  i += 1
end
