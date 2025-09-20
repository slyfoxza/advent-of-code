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
module y2015_d02;

import std.algorithm;
import std.conv;
import std.format;
import std.stdio;

import aoc;
import registry;

mixin CExtern!"c_y2015_d02";
mixin CppExtern!"cpp_y2015_d02";
mixin CExtern!"rust_y2015_d02";

static this() {
	register(2015, 2,
			new CSolution(&c_y2015_d02),
			new CppSolution(&cpp_y2015_d02),
			new DSolution(&d_y2015_d02),
			new RustSolution(&rust_y2015_d02),
	);
}

Answers d_y2015_d02() {
	uint paper, ribbon;
	foreach (line; File("y2015/d02/input").byLine) {
		uint l, w, h;
		line.formattedRead("%dx%dx%d", l, w, h);

		auto lw = l * w, wh = w * h, hl = h * l;
		paper += 2 * (lw + wh + hl) + min(lw, wh, hl);
		ribbon += 2 * (l + w + h - max(l, w, h)) + l * w * h;
	}

	return Answers(paper.to!string, ribbon.to!string);
}
