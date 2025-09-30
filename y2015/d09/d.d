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
module y2015_d09;

import std.array;
import std.algorithm;
import std.container.rbtree;
import std.conv;
import std.format;
import std.range;
import std.stdio;
import std.typecons;

import aoc;
import registry;

shared static this() {
	register(2015, 9,
			new DSolution(&d_y2015_d09),
	);
}

Answers d_y2015_d09() {
	alias Pair = Tuple!(string, string);
	uint[Pair] distances;

	auto locations = redBlackTree!string;
	foreach (line; File("y2015/d09/input").byLine) {
		string place1, place2;
		uint distance;
		line.formattedRead("%s to %s = %d", place1, place2, distance);
		locations.insert!string(place1);
		locations.insert!string(place2);

		if (place1 < place2) {
			distances[Pair(place1, place2)] = distance;
		} else {
			distances[Pair(place2, place1)] = distance;
		}
	}

	uint minDistance = uint.max;
	uint maxDistance = uint.min;
	foreach (permutation; array(locations).permutations) {
		uint totalDistance = 0;
		foreach (pair; permutation.slide(2)) {
			if (pair[0] < pair[1]) {
				totalDistance += distances[Pair(pair[0], pair[1])];
			} else {
				totalDistance += distances[Pair(pair[1], pair[0])];
			}
		}

		if (totalDistance < minDistance) {
			minDistance = totalDistance;
		}
		if (totalDistance > maxDistance) {
			maxDistance = totalDistance;
		}
	}

	return Answers(minDistance.to!string, maxDistance.to!string);
}
