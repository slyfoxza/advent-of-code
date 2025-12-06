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
module y2025_d06;

import std.algorithm;
import std.array;
import std.conv;
import std.stdio;
import std.typecons;

import aoc;
import registry;

// mixin CExtern!"c_y2025_d06";
// mixin CppExtern!"cpp_y2025_d06";
// mixin CExtern!"rust_y2025_d06";

shared static this() {
	register(2025, 6,
			// new CSolution(&c_y2025_d06),
			// new CppSolution(&cpp_y2025_d06),
			new DSolution(&d_y2025_d06),
			// new RustSolution(&rust_y2025_d06),
	);
}

alias Sums = Tuple!(long, "mulsum", long, "addsum");

Answers d_y2025_d06() {
	Sums[] sums = null;
	long grandTotal1 = 0;
	foreach (line; File("y2025/d06/input").byLine) {
		const bits = line.split;
		if (bits[0] == "*" || bits[0] == "+") {
			foreach (size_t i, operand; bits) {
				if (operand == "*") {
					grandTotal1 += sums[i].mulsum;
				} else if (operand == "+") {
					grandTotal1 += sums[i].addsum;
				}
			}
			break;
		}

		const values = bits.map!(a => a.parse!long).array;
		if (sums == null) {
			sums = new Sums[values.length];
			foreach (size_t i, value; values) {
				sums[i].mulsum = value;
				sums[i].addsum = value;
			}
		} else {
			foreach (size_t i, value; values) {
				sums[i].mulsum *= value;
				sums[i].addsum += value;
			}
		}
	}
	sums = null;

	long grandTotal2 = 0;
	const lines = File("y2025/d06/input").byLine.map!idup.array;
	long[] values;
	bool skip = false;
	foreach_reverse (i; 0 .. lines[0].length) {
		if (skip) {
			skip = false;
			continue;
		}

		char[] buffer;
		foreach (c; lines[0..$-1].map!(a => a[i])) {
			if (c == ' ') continue;
			buffer ~= c;
		}

		values ~= buffer.parse!long;

		if (lines[$-1][i] != ' ') {
			if (lines[$-1][i] == '*') {
				grandTotal2 += values.fold!((a, b) => a * b);
			} else if (lines[$-1][i] == '+') {
				grandTotal2 += values.sum;
			}
			values.length = 0;
			skip = true;
		}
	}

	return Answers(grandTotal1.to!string, grandTotal2.to!string);
}
