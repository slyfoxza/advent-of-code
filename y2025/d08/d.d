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
module y2025_d08;

import std.algorithm;
import std.conv;
import std.format;
import std.math;
import std.range;
import std.stdio;

import aoc;
import registry;

// mixin CExtern!"c_y2025_d08";
// mixin CppExtern!"cpp_y2025_d08";
// mixin CExtern!"rust_y2025_d08";

shared static this() {
	register(2025, 8,
			// new CSolution(&c_y2025_d08),
			// new CppSolution(&cpp_y2025_d08),
			new DSolution(&d_y2025_d08),
			// new RustSolution(&rust_y2025_d08),
	);
}

struct JunctionBox {
	int x, y, z;
	int circuit;
}

struct Connection {
	JunctionBox* jb1, jb2;
	float distance;
}

Answers d_y2025_d08() {
	JunctionBox[] boxes;
	int nextCircuit = 0;
	foreach (line; File("y2025/d08/input").byLine) {
		int x, y, z;
		line.formattedRead("%d,%d,%d", x, y, z);
		boxes ~= JunctionBox(x, y, z, nextCircuit++);
	}

	Connection[] connections;
	foreach (i, ref jb1; boxes.dropBackOne) {
		foreach (j, ref jb2; boxes.drop(i + 1)) {
			const distance = sqrt(
					pow(float(jb1.x) - jb2.x, 2) +
					pow(float(jb1.y) - jb2.y, 2) +
					pow(float(jb1.z) - jb2.z, 2)
			);
			connections ~= Connection(&jb1, &jb2, distance);
		}
	}

	connections.sort!"a.distance < b.distance";
	foreach (connection; connections.take(1000)) {
		const oldCircuit = max(connection.jb1.circuit, connection.jb2.circuit);
		const newCircuit = min(connection.jb1.circuit, connection.jb2.circuit);
		foreach (ref box; boxes) {
			if (box.circuit == oldCircuit) {
				box.circuit = newCircuit;
			}
		}
	}

	int[int] circuitSizes;
	foreach (box; boxes) {
		circuitSizes.require(box.circuit, 0);
		circuitSizes[box.circuit]++;
	}

	const sizeProduct = circuitSizes.byValue.array.sort!"a > b".take(3).fold!"a * b";

	long xmul;
	foreach (connection; connections) {
		if (connection.jb1.circuit == connection.jb2.circuit) {
			continue;
		}

		const oldCircuit = max(connection.jb1.circuit, connection.jb2.circuit);
		const newCircuit = min(connection.jb1.circuit, connection.jb2.circuit);
		foreach (ref box; boxes) {
			if (box.circuit == oldCircuit) {
				box.circuit = newCircuit;
			}
		}

		xmul = long(connection.jb1.x) * connection.jb2.x;
	}

	return Answers(sizeProduct.to!string, xmul.to!string);
}
