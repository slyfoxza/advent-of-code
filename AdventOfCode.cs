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
using System.Runtime.InteropServices;
using System.Text;

class AdventOfCode {
	public static void CopyAnswers(string answer1, nint buffer1, string answer2, nint buffer2,
			int bufferLength) {
		CopyAnswer(answer1, buffer1, bufferLength);
		CopyAnswer(answer2, buffer2, bufferLength);
	}

	public static void CopyAnswers(int answer1, nint buffer1, int answer2, nint buffer2,
			int bufferLength) {
		CopyAnswers(answer1.ToString(), buffer1, answer2.ToString(), buffer2, bufferLength);
	}

	private static void CopyAnswer(string answer, nint buffer, int bufferLength) {
		byte[] utf8 = Encoding.UTF8.GetBytes(answer);
		Marshal.Copy(utf8, 0, buffer, Math.Min(bufferLength, utf8.Length));
		// Ensure the result is null-terminated
		Marshal.WriteByte(buffer, Math.Min(bufferLength - 1, utf8.Length), 0x00);
	}
}
