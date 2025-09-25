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
require 'digest'

secret_key = File.read('y2015/d04/input').strip
part1 = 0
i = 1
loop do
  hash = Digest::MD5.digest("#{secret_key}#{i}").bytes
  if hash[0] == 0 && hash[1] == 0
    if part1 == 0 && (hash[2] & 0xF0) == 0
      part1 = i
    end
    if hash[2] == 0
      puts "#{part1} #{i}"
      break
    end
  end
  i += 1
end
