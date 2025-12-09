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
module y2025_d09;

import std.algorithm;
import std.conv;
import std.format;
import std.range;
import std.stdio;

import aoc;
import registry;

// mixin CExtern!"c_y2025_d09";
// mixin CppExtern!"cpp_y2025_d09";
// mixin CExtern!"rust_y2025_d09";

shared static this() {
	register(2025, 9,
			// new CSolution(&c_y2025_d09),
			// new CppSolution(&cpp_y2025_d09),
			new DSolution(&d_y2025_d09),
			// new RustSolution(&rust_y2025_d09),
	);
}

struct Vertex {
	int x, y;
}

Answers d_y2025_d09() {
	Vertex[] vertices;
	foreach (line; File("y2025/d09/input").byLine) {
		int x, y;
		line.formattedRead("%d,%d", x, y);
		vertices ~= Vertex(x, y);
	}

	long maxArea = 0;
	foreach (i, v1; vertices.dropBackOne) {
		foreach (v2; vertices.drop(i + 1)) {
			const x1 = min(v1.x, v2.x);
			const x2 = max(v1.x, v2.x);
			const y1 = min(v1.y, v2.y);
			const y2 = max(v1.y, v2.y);

			long w = x2 - x1 + 1;
			long h = y2 - y1 + 1;
			const area = w * h;
			maxArea = max(maxArea, area);
		}
	}

	return Answers(maxArea.to!string, "?");
}
