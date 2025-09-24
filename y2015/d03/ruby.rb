# Copyright 2025 Philip Cronje
#
# This file is part of my Advent of Code solution repository. It is free software: you can
# redistribute it and/or modify it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or (at your option) any later
# version.
#
# This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with this repository. If
# not, see <https://www.gnu.org/licenses/>.
Point = Struct.new('Point', :x, :y)

solo_santa_pos = Point.new(0, 0)

santa_pos = Point.new(0, 0)
robo_pos = Point.new(0, 0)
current = robo_pos

visited1 = Hash.new
visited2 = Hash.new

visited1[solo_santa_pos] = true
visited2[santa_pos] = true

File.open('y2015/d03/input').each_char do |c|
  solo_santa_pos = solo_santa_pos.dup
  current = if current.object_id == robo_pos.object_id
              santa_pos = santa_pos.dup
            else
              robo_pos = robo_pos.dup
            end

  case c
  when '^'
    solo_santa_pos.y -= 1
    current.y -= 1
  when 'v'
    solo_santa_pos.y += 1
    current.y += 1
  when '<'
    solo_santa_pos.x -= 1
    current.x -= 1
  when '>'
    solo_santa_pos.x += 1
    current.x += 1
  end

  visited1[solo_santa_pos] = true
  visited2[current] = true
end

puts "#{visited1.size} #{visited2.size}"
