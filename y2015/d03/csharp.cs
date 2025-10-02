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
using System.Drawing;
using System.IO;
using System.Runtime.InteropServices;

class CSharpY2015D03 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		var visited1 = new HashSet<Point>();
		var visited2 = new HashSet<Point>();

		var soloSantaPos = new Point(0, 0);

		var santaPos = new Point(0, 0);
		var roboPos = new Point(0, 0);
		bool isRobo = true;

		visited1.Add(soloSantaPos);
		visited2.Add(santaPos);

		using StreamReader reader = File.OpenText("y2015/d03/input");
		int c;
		while ((c = reader.Read()) != -1) {
			ref var current = ref (isRobo ? ref santaPos : ref roboPos);
			isRobo = !isRobo;

			if (c == '^') {
				soloSantaPos.Offset(0, -1);
				current.Offset(0, -1);
			} else if (c == 'v') {
				soloSantaPos.Offset(0, 1);
				current.Offset(0, 1);
			} else if (c == '<') {
				soloSantaPos.Offset(-1, 0);
				current.Offset(-1, 0);
			} else if (c == '>') {
				soloSantaPos.Offset(1, 0);
				current.Offset(1, 0);
			}

			visited1.Add(soloSantaPos);
			visited2.Add(current);
		}

		AdventOfCode.CopyAnswers(visited1.Count.ToString(), buf1, visited2.Count.ToString(), buf2,
				buflen);
	}
}
