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
module y2015_d04;

import std.conv;
import std.digest.md;
import std.stdio;
import std.string;

import aoc;
import registry;

shared static this() {
	register(2015, 4,
			new DSolution(&d_y2015_d04),
	);
}

Answers d_y2015_d04() {
	uint part1 = 0, part2 = 0;
	auto secretKey = File("y2015/d04/input").readln().stripRight;
	for (uint i = 1; part2 == 0 || part1 == 0; i++) {
		auto hash = md5Of(secretKey ~ i.to!string);
		if (part1 == 0 && hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0) {
			part1 = i;
		}
		if (part2 == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] == 0) {
			part2 = i;
		}
	}

	return Answers(part1.to!string, part2.to!string);
}
