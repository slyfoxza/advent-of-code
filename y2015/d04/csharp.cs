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
using System.Security.Cryptography;
using System.Text;

class CSharpY2015D04 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		int part1 = 0, part2 = 0;
		string secretKey = File.ReadAllText("y2015/d04/input").TrimEnd();
		for (int i = 0; part2 == 0 || part1 == 0; i++) {
			byte[] hash = MD5.HashData(Encoding.UTF8.GetBytes(secretKey + i.ToString()));
			if (part1 == 0 && hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0) {
				part1 = i;
			}
			if (part2 == 0 && hash[0] == 0 && hash[1] == 0 && hash[2] == 0) {
				part2 = i;
			}
		}

		AdventOfCode.CopyAnswers(part1.ToString(), buf1, part2.ToString(), buf2, buflen);
	}
}
