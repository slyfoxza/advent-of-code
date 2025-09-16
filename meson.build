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
project(
  'advent-of-code',
  'c', 'cpp', 'd', 'rust',
  default_options : {
    'buildtype': 'debugoptimized',
    'rust_std': '2024',
  },
  meson_version : '>=1.3.0'
)
rust_lib = static_library('advent-of-rust', 'aoc_rs.rs', rust_abi : 'c')
executable(
  'advent-of-code',
  'main.d', 'aoc.d', 'python.c', 'registry.d',
  'y2015/d01/d.d', 'y2015/d01/c.c', 'y2015/d01/cpp.cpp',
  dependencies : [dependency('guile-3.0'), dependency('lua'), dependency('python3-embed')],
  link_with : rust_lib
)
