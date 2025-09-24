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
module y2015_d03;

import std.conv;
import std.complex;
import std.stdio;

import aoc;
import registry;

mixin CExtern!"c_y2015_d03";
mixin CppExtern!"cpp_y2015_d03";
mixin CExtern!"rust_y2015_d03";

static this() {
	register(2015, 3,
			new CSolution(&c_y2015_d03),
			new CppSolution(&cpp_y2015_d03),
			new DSolution(&d_y2015_d03),
			new RustSolution(&rust_y2015_d03),
	);
}

struct Point {
	int x, y;
}

Answers d_y2015_d03() {
	bool[Point] visited1, visited2;
	auto soloSantaPos = Point(0, 0);

	auto santaPos = Point(0, 0);
	auto roboPos = Point(0, 0);
	Point* current = &roboPos;

	visited1[soloSantaPos] = true;
	visited2[santaPos] = true;

	foreach (buffer; File("y2015/d03/input").byChunk(4096)) {
		foreach (c; buffer) {
			current = current == &roboPos ? &santaPos : &roboPos;

			if (c == '^') {
				soloSantaPos.y--;
				current.y--;
			}
			else if (c == 'v') {
				soloSantaPos.y++;
				current.y++;
			}
			else if (c == '<') {
				soloSantaPos.x--;
				current.x--;
			}
			else if (c == '>') {
				soloSantaPos.x++;
				current.x++;
			}

			visited1[soloSantaPos] = true;
			visited2[*current] = true;
		}
	}

	return Answers(visited1.length.to!string, visited2.length.to!string);
}
