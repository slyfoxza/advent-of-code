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

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.stream.Stream;

import aoc.AdventOfCode.Answers;
import aoc.AdventOfCode.Solution;

public class JavaY2015D02 implements Solution {
	private static record State(int paper, int ribbon) {}

	@Override
	public Answers solve() {
		try (final Stream<String> lines = Files.lines(Paths.get("y2015/d02/input"))) {
			return lines.map((line) -> {
				String[] bits = line.split("x", 3);
				int[] dimensions = new int[bits.length];
				Arrays.setAll(dimensions, (i) -> Integer.valueOf(bits[i]));
				int l = dimensions[0], w = dimensions[1], h = dimensions[2];
				int lw = l * w, wh = w * h, hl = h * l;
				return new State(
						2 * (lw + wh + hl) + Math.min(Math.min(lw, wh), hl),
						2 * (l + w + h - Math.max(Math.max(l, w), h)) + l * w * h
				);
			}).reduce((a, b) -> {
				return new State(a.paper() + b.paper(), a.ribbon() + b.ribbon());
			}).map((x) -> new Answers(x.paper(), x.ribbon()))
				.orElse(new Answers(0, 0));
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}
}
