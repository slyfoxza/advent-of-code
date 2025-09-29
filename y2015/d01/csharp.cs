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
using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

class CSharpY2015D01 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		int floor = 0, position = -1, i = 0;

		using StreamReader reader = File.OpenText("y2015/d01/input");
		int c;
		while((c = reader.Read()) != -1) {
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

		AdventOfCode.CopyAnswers(floor, buf1, position, buf2, buflen);
	}
}
