/* Copyright 2025 Philip Cronje
 *
 * This file is part of my Advent of Code solution repository. It is free software: you can
 * redistribute it and/or modify it under the terms of the GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This repository is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this repository. If
 * not, see <https://www.gnu.org/licenses/>. */
module y2015_d01;

import std.conv;
import std.stdio;

import aoc;
import registry;

mixin CExtern!"c_y2015_d01";
mixin CppExtern!"cpp_y2015_d01";
mixin CExtern!"rust_y2015_d01";

static this() {
	register(2015, 1,
			new CSolution(&c_y2015_d01),
			new CppSolution(&cpp_y2015_d01),
			new DSolution(&d_y2015_d01),
			new RustSolution(&rust_y2015_d01),
	);
}

Answers d_y2015_d01() {
	int floor, position = -1;
	uint i;
	foreach (buffer; File("y2015/d01/input").byChunk(4096)) {
		foreach (c; buffer) {
			if (c == '(') {
				floor++;
			} else if (c == ')') {
				floor--;
			}

			if (i++ && position == -1 && floor < 0) {
				position = i;
			}
		}
	}

	return Answers(floor.to!string, position.to!string);
}
