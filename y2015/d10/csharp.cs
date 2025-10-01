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
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

class CSharpY2015D10 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		int part1 = 0;

		string input = File.ReadAllText("y2015/d10/input");
		StringBuilder sb = new StringBuilder();
		for (int i = 0; i < 50; i++) {
			if (i == 40) {
				part1 = input.Length;
			}

			int n = 0;
			char pc = '\0';
			foreach (char c in input) {
				if (c != pc && n > 0) {
					sb.Append(n);
					sb.Append(pc);
					n = 0;
				}

				pc = c;
				n++;
			}

			sb.Append(n);
			sb.Append(pc);

			input = sb.ToString();
			sb.Clear();
		}
		AdventOfCode.CopyAnswers(part1.ToString(), buf1, input.Length.ToString(), buf2, buflen);
	}
}
