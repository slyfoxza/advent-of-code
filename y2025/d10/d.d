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
module y2025_d10;

import std.algorithm;
import std.array;
import std.container;
import std.conv;
import std.stdio;
import std.string;

import aoc;
import registry;

// mixin CExtern!"c_y2025_d10";
// mixin CppExtern!"cpp_y2025_d10";
// mixin CExtern!"rust_y2025_d10";

shared static this() {
	register(2025, 10,
			// new CSolution(&c_y2025_d10),
			// new CppSolution(&cpp_y2025_d10),
			new DSolution(&d_y2025_d10),
			// new RustSolution(&rust_y2025_d10),
	);
}

struct Machine {
	char[] desiredLights;
	int[][] buttons;
	int[] requiredJoltages;
}

struct State {
	char[] lights;
	int buttonsPressed = 0;
}

Answers d_y2025_d10() {
	Machine[] machines;
	foreach (line; File("y2025/d10/input").byLine) {
		auto close = line.indexOf(']');
		const lights = line[1 .. close];

		Machine machine = Machine(lights.dup);

		line = line[close+1 .. $];
		ptrdiff_t open;
		while ((open = line.indexOf('(')) != -1) {
			close = line.indexOf(')');
			const csv = line[open+1 .. close];
			machine.buttons ~= csv.split(',').map!(to!int).array;

			line = line[close+1 .. $];
		}

		open = line.indexOf('{');
		close = line.indexOf('}');
		machine.requiredJoltages = line[open+1 .. close].split(',').map!(to!int).array;

		machines ~= machine;
	}

	int buttonsPressed = 0;
	DList!State queue;
	foreach (machine; machines) {
		char[] lights = new char[machine.desiredLights.length];
		lights.fill('.');
		queue.insertBack(State(lights));

		bfs:
		while (!queue.empty) {
			const state = queue.front;
			queue.removeFront();

			foreach (buttons; machine.buttons) {
				auto nextLights = state.lights.dup;
				foreach (button; buttons) {
					nextLights[button] = nextLights[button] == '#' ? '.' : '#';
				}

				if (nextLights == machine.desiredLights) {
					buttonsPressed += state.buttonsPressed + 1;
					break bfs;
				}

				queue.insertBack(State(nextLights, state.buttonsPressed + 1));
			}
		}
		queue.clear();
	}

	return Answers(buttonsPressed.to!string, "?");
}
