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
module y2025_d12;

import std.algorithm;
import std.conv;
import std.format;
import std.stdio;
import std.string;

import aoc;
import registry;

// mixin CExtern!"c_y2025_d12";
// mixin CppExtern!"cpp_y2025_d12";
// mixin CExtern!"rust_y2025_d12";

shared static this() {
	register(2025, 12,
			// new CSolution(&c_y2025_d12),
			// new CppSolution(&cpp_y2025_d12),
			new DSolution(&d_y2025_d12),
			// new RustSolution(&rust_y2025_d12),
	);
}

struct Present {
	uint size;
}

Answers d_y2025_d12() {
	Present[] presents;
	Present present;
	uint validRegions = 0;
	foreach (line; File("y2025/d12/input").byLine) {
		const colon = line.indexOf(':');
		const x = line.indexOf('x');
		if (colon != -1 && x != -1) {
			// area line
			uint w, h;
			int consumedSize = 0;
			line[0 .. colon].formattedRead("%dx%d", w, h);
			uint area = w * h;
			foreach (i, s; line[colon + 2 .. $].split) {
				const n = s.to!int;
				consumedSize += n * presents[i].size;
				if (consumedSize > area) {
					consumedSize = -1;
					break;
				}
			}

			if (consumedSize != -1) {
				validRegions++;
			}
		} else if (colon != -1) {
			// grid start
			present = Present();
		} else if (line.length == 0) {
			presents ~= present;
		} else {
			// grid contents
			foreach (c; line) {
				if (c == '#') {
					present.size++;
				}
			}
		}
	}

	return Answers(validRegions.to!string, "-");
}
