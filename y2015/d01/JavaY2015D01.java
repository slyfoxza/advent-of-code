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
package aoc.solutions;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import aoc.AdventOfCode.Answers;
import aoc.AdventOfCode.Solution;

public class JavaY2015D01 implements Solution {
	@Override
	public Answers solve() {
		int floor = 0, position = -1, i = 0;

		try (final BufferedReader reader = Files.newBufferedReader(Paths.get("y2015/d01/input"))) {
			int c;
			while ((c = reader.read()) != -1) {
				if (c == '(') {
					floor++;
				} else if (c == ')') {
					floor--;
				}

				i++;
				if (position == -1 && floor < 0) {
					position = i;
				}
			}

			return new Answers(Integer.toString(floor), Integer.toString(position));
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}
}
