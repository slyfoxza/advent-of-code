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
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;

class CSharpY2025D04 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		List<char[]> grid = new List<char[]>();

		using StreamReader reader = File.OpenText("y2025/d04/input");
		string line;
		while ((line = reader.ReadLine()) != null) {
			grid.Add(line.ToCharArray());
		}

		int accessible = RemoveRolls(grid);
		int totalRemoved = accessible;
		int removed;
		while ((removed = RemoveRolls(grid)) != 0) {
			totalRemoved += removed;
		}

		AdventOfCode.CopyAnswers(accessible, buf1, totalRemoved, buf2, buflen);
	}

	private static int RemoveRolls(List<char[]> grid) {
		int w = grid[0].Length;
		int h = grid.Count;

		int removed = 0;
		for (int y = 0; y < h; ++y) {
			for (int x = 0; x < w; ++x) {
				if (grid[y][x] != '@') continue;

				int adjCount = 0;
				for (int dx = -1; dx <= 1; ++dx) {
					for (int dy = -1; dy <= 1; ++dy) {
						if (dx == 0 && dy == 0) continue;

						int x1 = x + dx;
						int y1 = y + dy;
						if (x1 < 0 || x1 >= w || y1 < 0 || y1 >= h) continue;

						if (grid[y1][x1] != '.') adjCount++;
						if (adjCount >= 4) break;
					}
					if (adjCount >= 4) break;
				}

				if (adjCount < 4) {
					grid[y][x] = 'x';
					removed++;
				}
			}
		}

		for (int y = 0; y < h; ++y) {
			for (int x = 0; x < w; ++x) {
				if (grid[y][x] == 'x') grid[y][x] = '.';
			}
		}

		return removed;
	}
}
