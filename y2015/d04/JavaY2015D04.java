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
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

import aoc.AdventOfCode.Answers;
import aoc.AdventOfCode.Solution;

public class JavaY2015D04 implements Solution {
	@Override
	public Answers solve() {
		try {
			final MessageDigest md5 = MessageDigest.getInstance("MD5");

			int part1 = 0, part2 = 0;
			final String secretKey = Files.readString(Paths.get("y2015/d04/input")).stripTrailing();
			for (int i = 0; part2 == 0 || part1 == 0; i++) {
				final byte[] hash = md5.digest((secretKey + Integer.toString(i)).getBytes(StandardCharsets.UTF_8));
				if (part1 == 0 && hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0) {
					part1 = i;
				}
				if (part2 == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] == 0) {
					part2 = i;
				}
			}

			return new Answers(Integer.toString(part1), Integer.toString(part2));
		} catch (final NoSuchAlgorithmException e) {
			throw new RuntimeException(e);
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}
	}
}
