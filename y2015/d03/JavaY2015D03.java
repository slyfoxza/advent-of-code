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
import java.util.HashSet;
import java.util.Set;

import aoc.AdventOfCode.Answers;
import aoc.AdventOfCode.Solution;

public class JavaY2015D03 implements Solution {
	private static record Point(int x, int y) {
		private Point up() {
			return new Point(x, y - 1);
		}

		private Point down() {
			return new Point(x, y + 1);
		}

		private Point left() {
			return new Point(x - 1, y);
		}

		private Point right() {
			return new Point(x + 1, y);
		}
	}

	@Override
	public Answers solve() {
		final Set<Point> visited1 = new HashSet<>();
		final Set<Point> visited2 = new HashSet<>();

		Point soloSantaPos = new Point(0, 0);

		Point santaPos = new Point(0, 0);
		Point roboPos = new Point(0, 0);
		boolean isRobo = true;

		visited1.add(soloSantaPos);
		visited2.add(santaPos);

		try (final BufferedReader reader = Files.newBufferedReader(Paths.get("y2015/d03/input"))) {
			int c;
			while ((c = reader.read()) != -1) {
				final Point current = isRobo ? santaPos : roboPos;
				isRobo = !isRobo;

				Point next = null;
				if (c == '^') {
					soloSantaPos = soloSantaPos.up();
					next = current.up();
				} else if (c == 'v') {
					soloSantaPos = soloSantaPos.down();
					next = current.down();
				} else if (c == '<') {
					soloSantaPos = soloSantaPos.left();
					next = current.left();
				} else if (c == '>') {
					soloSantaPos = soloSantaPos.right();
					next = current.right();
				}

				if (isRobo) {
					roboPos = next;
				} else {
					santaPos = next;
				}

				visited1.add(soloSantaPos);
				visited2.add(next);
			}

			return new Answers(visited1.size(), visited2.size());
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}
}
