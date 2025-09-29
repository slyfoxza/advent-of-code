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

class CSharpY2015D02 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		int paper = 0, ribbon = 0;

		using StreamReader reader = File.OpenText("y2015/d02/input");
		string line;
		while ((line = reader.ReadLine()) != null) {
			string[] dimensions = line.Split('x', 3);
			int l = int.Parse(dimensions[0]);
			int w = int.Parse(dimensions[1]);
			int h = int.Parse(dimensions[2]);

			int lw = l * w, wh = w * h, hl = h * l;
			paper += 2 * (lw + wh + hl) + Math.Min(Math.Min(lw, wh), hl);
			ribbon += 2 * (l + w + h - Math.Max(Math.Max(l, w), h)) + l * w * h;
		}

		AdventOfCode.CopyAnswers(paper, buf1, ribbon, buf2, buflen);
	}
}
