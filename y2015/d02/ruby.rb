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
paper = ribbon = 0
File.open('y2015/d02/input').each_line do |line|
  l, w, h = line.split('x').map(&:to_i)
  lw, wh, hl = l * w, w * h, h * l
  paper += 2 * (lw + wh + hl) + [lw, wh, hl].min()
  ribbon += 2 * (l + w + h - [l, w, h].max()) + l * w * h
end

puts "#{paper} #{ribbon}"
