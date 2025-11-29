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
using System.Text.Json;

class CSharpY2015D12 {
	[UnmanagedCallersOnlyAttribute]
	public static void Solve(nint buf1, nint buf2, int buflen) {
		int allSum = 0;
		int filteredSum = 0;
		Stack<int> stack = new Stack<int>();

		bool checkProperty = false;
		Utf8JsonReader reader = new Utf8JsonReader(File.ReadAllBytes("y2015/d12/input"));
		while (reader.Read()) {
			if (reader.TokenType == JsonTokenType.StartObject) {
				stack.Push(filteredSum);
			} else if (reader.TokenType == JsonTokenType.EndObject) {
				stack.Pop();
			} else if (reader.TokenType == JsonTokenType.PropertyName) {
				checkProperty = true;
				continue;
			} else if (checkProperty && reader.TokenType == JsonTokenType.String) {
				if (reader.ValueTextEquals("red")) {
					int level = 0;
					while (level != 0 || reader.TokenType != JsonTokenType.EndObject) {
						if (reader.TokenType == JsonTokenType.StartObject) {
							level++;
						} else if (reader.TokenType == JsonTokenType.EndObject) {
							level--;
						} else if (reader.TokenType == JsonTokenType.Number) {
							allSum += reader.GetInt32();
						}
						reader.Read();
					}
					filteredSum = stack.Pop();
				}
			} else if (reader.TokenType == JsonTokenType.Number) {
				int value = reader.GetInt32();
				allSum += value;
				filteredSum += value;
			}

			checkProperty = false;
		}

		AdventOfCode.CopyAnswers(allSum.ToString(), buf1, filteredSum.ToString(), buf2, buflen);
	}
}
