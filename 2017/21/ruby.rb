#!/usr/bin/env ruby
def invert(from)
  r = []
  (0...from.length).each do |y|
    r[y] = ''
    (0...from.length).each do |z|
      r[y] << from[z][y]
    end
  end
  r
end
def flip(from)
  i = from.length - 1
  r = Array.new(from)
  r[0] = from[i]
  r[i] = from[0]
  r
end
twos = {}; threes = {}
$stdin.each_line
  .map { |x| %r{([.#/]+) => ([.#/]+)}.match x }
  .each do |x|
    from, to = x[1..2].map { |x| x.split '/' }
    froms = []
    4.times do
      from = invert(from); froms << from
      from = flip(from); froms << from
    end
    froms.each do |f|
      if f.length == 2
        twos[f.join ''] = to
      else
        threes[f.join ''] = to
      end
    end
  end
size = 3
image = '.#...####'
5.times do
  q, r = size.divmod 2
  if r == 0
    rule = twos
    subsz = 2
  else
    rule = threes
    subsz = 3
    q = size / 3
  end
  new_image = []
  q.times do |i|
    q.times do |j|
      subsq = (0...subsz).map { |y| image[(j * subsz + y) * size + i * subsz, subsz] }.join ''
      repl = rule[subsq]
      (subsz + 1).times do |y|
        new_image[j * subsz.next + y] ||= ''
        new_image[j * subsz.next + y] << repl[y]
      end
    end
  end
  image = new_image.join ''
  size += q
end
puts image.count '#'
