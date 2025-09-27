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
module y2015_d06;

import std.conv;
import std.format;
import std.stdio;

import aoc;
import registry;

shared static this() {
	register(2015, 6,
			new DSolution(&d_y2015_d06),
	);
}

immutable TurnOn = "turn on";
immutable TurnOff = "turn off";
immutable Toggle = "toggle";

Answers d_y2015_d06() {
	auto binaryLights = new bool[][](1000, 1000);
	auto brightLights = new ushort[][](1000, 1000);

	foreach (line; File("y2015/d06/input").byLine) {
		string action;
		ushort x0, y0, x1, y1;

		if (line[0 .. TurnOff.length] == TurnOff) {
			action = TurnOff;
		} else if (line[0 .. TurnOn.length] == TurnOn) {
			action = TurnOn;
		} else {
			action = Toggle;
		}

		line[action.length + 1 .. $].formattedRead("%d,%d through %d,%d", x0, y0, x1, y1);
		for (ushort x = x0; x <= x1; x++) {
			for (ushort y = y0; y <= y1; y++) {
				if (action == TurnOff) {
					binaryLights[x][y] = false;
					if (brightLights[x][y] > 0) {
						brightLights[x][y]--;
					}
				} else if (action == TurnOn) {
					binaryLights[x][y] = true;
					brightLights[x][y]++;
				} else {
					binaryLights[x][y] = !binaryLights[x][y];
					brightLights[x][y] += 2;
				}
			}
		}
	}

	uint lightsOn = 0, brightness = 0;
	for (ushort x = 0; x < 1000; x++) {
		for (ushort y = 0; y < 1000; y++) {
			if (binaryLights[x][y]) lightsOn++;
			brightness += brightLights[x][y];
		}
	}

	return Answers(lightsOn.to!string, brightness.to!string);
}
