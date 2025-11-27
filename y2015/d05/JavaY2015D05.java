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
import java.util.stream.Stream;

import aoc.AdventOfCode.Answers;
import aoc.AdventOfCode.Solution;

public class JavaY2015D05 implements Solution {
	private static record State(int nice1, int nice2) {}

	@Override
	public Answers solve() {
		try (final Stream<String> lines = Files.lines(Paths.get("y2015/d05/input"))) {
			return lines.map((line) -> {
				int vowels = 0;
				boolean doubled = false;
				boolean forbiddenPair = false;

				boolean doublePair = false;
				boolean sandwichPair = false;

				for (int i = 0; i < line.length(); i++) {
					char c0 = line.charAt(i);
					if (isVowel(c0)) vowels++;

					if (i < line.length() - 1) {
						char c1 = line.charAt(i + 1);
						if ((c0 == 'a' || c0 == 'c' || c0 == 'p' || c0 == 'x') && c1 == c0 + 1) {
							forbiddenPair = true;
						} else if (c0 == c1) {
							doubled = true;
						}

						if (!doublePair) {
							String needle = line.substring(i, i + 2);
							if (line.substring(i + 2).contains(needle)) {
								doublePair = true;
							}
						}
					}
					if (i < line.length() - 2) {
						char c2 = line.charAt(i + 2);
						if (c0 == c2) {
							sandwichPair = true;
						}
					}
				}

				return new State(
						doubled && !forbiddenPair && vowels >= 3 ? 1 : 0,
						doublePair && sandwichPair ? 1 : 0
				);
			}).reduce((a, b) -> {
				return new State(a.nice1() + b.nice1(), a.nice2() + b.nice2());
			}).map((x) -> new Answers(x.nice1(), x.nice2()))
				.orElse(new Answers(0, 0));
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}

	private static boolean isVowel(final char c) {
		return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
	}
}
