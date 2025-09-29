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
module y2015_d08;

import std.conv;
import std.stdio;

import aoc;
import registry;

shared static this() {
	register(2015, 8,
			new DSolution(&d_y2015_d08),
	);
}

Answers d_y2015_d08() {
	uint part1, part2;

	foreach (line; File("y2015/d08/input").byLine) {
		immutable codeLength = line.length;
		uint memLength = 0;
		for (int i = 1; i < codeLength - 1; i++) {
			memLength++;
			if (line[i] == '\\') {
				if (line[i + 1] == 'x') {
					i += 3;
				} else {
					i++;
				}
			}
		}
		part1 += codeLength - memLength;

		uint encodedLength = 2;
		for (int i = 0; i < codeLength; i++) {
			encodedLength++;
			if (line[i] == '"' || line[i] == '\\') {
				encodedLength++;
			}
		}
		part2 += encodedLength - codeLength;
	}

	return Answers(part1.to!string, part2.to!string);
}
