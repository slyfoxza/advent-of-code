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

class CSharpY2015D11 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		string password1 = GetNextPassword(File.ReadAllText("y2015/d11/input"));
		string password2 = GetNextPassword(password1);

		AdventOfCode.CopyAnswers(password1, buf1, password2, buf2, buflen);
	}

	private static string GetNextPassword(string password) {
		StringBuilder sb = new StringBuilder(password);
		while (true) {
			for (int i = sb.Length - 1; i >= 0; i--) {
				sb[i]++;

				if (sb[i] == '{') {
					sb[i] = 'a';
					if (i == 0) {
						sb.Insert(0, 'a');
						break;
					}
				} else if (sb[i] == 'i' || sb[i] == 'o' || sb[i] == 'l') {
					sb[i]++;
					for (int j = i + 1; j < sb.Length; j++) {
						sb[j] = 'a';
					}
					break;
				} else {
					break;
				}
			}

			bool hasStraight = false;
			int straight = 1;
			for (int i = 1; i < sb.Length; i++) {
				if (sb[i - 1] == sb[i] - 1) {
					if (++straight == 3) {
						hasStraight = true;
						break;
					}
				} else {
					straight = 1;
				}
			}

			int pairs = 0;
			if (hasStraight) {
				for (int i = 1; i < sb.Length; i++) {
					if (sb[i - 1] == sb[i]) {
						pairs++;
						i++;
					}
				}
			}

			if (hasStraight && pairs >= 2) {
				return sb.ToString();
			}
		}
	}
}
