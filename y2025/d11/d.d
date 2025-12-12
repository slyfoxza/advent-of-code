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
module y2025_d11;

import std.algorithm;
import std.array;
import std.container;
import std.conv;
import std.stdio;
import std.string;

import aoc;
import registry;

// mixin CExtern!"c_y2025_d11";
// mixin CppExtern!"cpp_y2025_d11";
// mixin CExtern!"rust_y2025_d11";

shared static this() {
	register(2025, 11,
			// new CSolution(&c_y2025_d11),
			// new CppSolution(&cpp_y2025_d11),
			new DSolution(&d_y2025_d11),
			// new RustSolution(&rust_y2025_d11),
	);
}

struct Node {
	string id;
	string[] targets;
}

Answers d_y2025_d11() {
	Node[] nodes;
	ulong[string] nodeIndex;
	foreach (line; File("y2025/d11/input").byLine) {
		const colonIndex = line.indexOf(':');
		const id = line[0 .. colonIndex];
		const targets = line[colonIndex + 2 .. $].split;

		auto node = Node(id.dup);
		nodeIndex[node.id] = nodes.length;
		node.targets.length = targets.length;
		foreach (i, target; targets) {
			node.targets[i] = target.dup;
		}
		nodes ~= node;
	}

	uint numPaths = 0;
	SList!string stack;
	stack.insertFront("you");
	while (!stack.empty) {
		const id = stack.front;
		stack.removeFront();

		const ref node = nodes[nodeIndex[id]];
		foreach (target; node.targets) {
			if (target == "out") {
				numPaths++;
			} else {
				stack.insertFront(target);
			}
		}
	}

	return Answers(numPaths.to!string, "tbd");
}
