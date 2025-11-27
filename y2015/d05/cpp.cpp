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
#include <cstring>
#include <fstream>

bool isVowel(char c) {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

void cpp_y2015_d05(char* buf1, char* buf2, size_t buflen) {
	unsigned nice = 0, nice2 = 0;

	std::ifstream f("y2015/d05/input");
	for (std::string line; std::getline(f, line);) {
		unsigned vowels = 0;
		bool doubled = false;
		bool forbiddenPair = false;

		bool doublePair = false;
		bool sandwichPair = false;

		if (isVowel(line[0])) vowels++;
		for (auto c = line.cbegin(); c != line.cend() - 1; c++) {
			char c0 = *c;
			char c1 = *(c + 1);
			if ((c0 == 'a' || c0 == 'c' || c0 == 'p' || c0 == 'x') && c1 == c0 + 1) {
				forbiddenPair = true;
			}

			if (isVowel(c1)) vowels++;
			if (c0 == c1) doubled = true;

			if (c0 == *(c + 2)) {
				sandwichPair = true;
			}

			char needle[3] = {c0, c1, '\0'};
			if (!doublePair && line.find(needle, c - line.cbegin() + 2) != std::string::npos) {
				doublePair = true;
			}
		}

		if (doubled && !forbiddenPair && vowels >= 3) {
			nice++;
		}
		if (doublePair && sandwichPair) {
			nice2++;
		}
	}

	strncpy(buf1, std::to_string(nice).c_str(), buflen);
	strncpy(buf2, std::to_string(nice2).c_str(), buflen);
}
