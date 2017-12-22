#!/usr/bin/env ruby
program = $stdin.each_line.map { |x| x.split }

last_freq = nil
recover = nil
ip = [0, 0]
regs = [Hash.new(0), Hash.new(0)]
(0..1).each { |p| regs[p][:p] = p }
queue = [[], []]
n_send = 0
until ip.all? { |x| x < 0 || x >= program.length }
  blocked = [false, false]
  (0..1).each do |p|
    if ip[p] < 0 || ip[p] >= program.length
      blocked[p] = true
      next
    end

    ins, x, y = program[ip[p]]
    unless y.nil?
      y = regs[p][y.to_sym] if y =~ /[a-z]/
      y = y.to_i
    end

    case ins
    when 'snd'
      x = regs[p][x.to_sym] if x =~ /[a-z]/
      last_freq = x.to_i if p == 0
      queue[p ^ 1] << x
      n_send += 1 if p == 1
    when 'rcv'
      if recover.nil? && p == 0
        xv = regs[p][x.to_sym] if x =~ /[a-z]/
        recover = last_freq if xv.to_i != 0
      end
      if queue[p].empty?
        blocked[p] = true
        next
      else
        regs[p][x.to_sym] = queue[p].delete_at 0
      end
    when 'set'; regs[p][x.to_sym] = y
    when 'add'; regs[p][x.to_sym] += y
    when 'mul'; regs[p][x.to_sym] *= y
    when 'mod'; regs[p][x.to_sym] %= y
    when 'jgz'
      x = regs[p][x.to_sym] if x=~ /[a-z]/
      if x.to_i > 0
        ip[p] += y
        next
      end
    end
    ip[p] += 1
  end
  break if blocked.all?
end

puts "#{recover} #{n_send}"
