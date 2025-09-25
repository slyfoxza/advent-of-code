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
module y2015_d05;

import std.algorithm;
import std.conv;
import std.range;
import std.stdio;

import aoc;
import registry;

mixin CExtern!"c_y2015_d05";

shared static this() {
	register(2015, 5,
			new CSolution(&c_y2015_d05),
			new DSolution(&d_y2015_d05),
	);
}

private bool isVowel(T)(T c) {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

Answers d_y2015_d05() {
	uint nice = 0, nice2 = 0;

	foreach (line; File("y2015/d05/input").byLine) {
		uint vowels = 0;
		bool doubled = false;
		bool forbiddenPair = false;

		bool doublePair = false;
		bool sandwichPair = false;

		if (line[0].isVowel) vowels++;
		foreach (pair; line.slide(2)) {
			const c0 = pair.front;
			pair.popFront();

			if ((c0 == 'a' || c0 == 'c' || c0 == 'p' || c0 == 'x') && pair.front == c0 + 1) {
				forbiddenPair = true;
			}

			if (pair.front.isVowel) vowels++;
			if (c0 == pair.front) doubled = true;

			// part 2 check
			dstring needle = [c0, pair.front];
			if (line.count(needle) >= 2) {
				doublePair = true;
			}
		}

		foreach (triplet; line.slide(3)) {
			const c0 = triplet.front; triplet.popFront();
			const c1 = triplet.front; triplet.popFront();

			if (c0 == triplet.front) {
				sandwichPair = true;
				break;
			}
		}

		if (doubled && !forbiddenPair && vowels >= 3) {
			nice++;
		}

		if (doublePair && sandwichPair) {
			nice2++;
		}
	}

	return Answers(nice.to!string, nice2.to!string);
}
