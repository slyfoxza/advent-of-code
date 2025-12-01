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
module y2025_d01;

import std.conv;
import std.format;
import std.math;
import std.stdio;

import aoc;
import registry;

shared static this() {
	register(2025, 1,
			new DSolution(&d_y2025_d01),
	);
}

Answers d_y2025_d01() {
	uint zeroes = 0, clicks = 0;
	int dial = 50;
	foreach (line; File("y2025/d01/input").byLine) {
		char dir;
		int sign = 1;
		int value;
		line.formattedRead("%c%d", dir, value);

		while (value > 100) {
			value -= 100;
			clicks++;
		}

		if (dir == 'L') {
			sign = -1;
		}
		int dialStart = dial;
		int dialNoMod = dial + (sign * value);
		dial = dialNoMod % 100;
		while (dial < 0) {
			dial += 100;
		}

		if (dial == 0) {
			zeroes++;
		}

		if (dialStart != 0 && sign == -1 && value >= dialStart) {
			clicks++;
		} else if (sign == 1 && value >= (100 - dialStart)) {
			clicks++;
		}
	}

	return Answers(zeroes.to!string, clicks.to!string);
}
