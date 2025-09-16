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

void cpp_y2015_d01(char* buf1, char* buf2, size_t buflen) {
	int floor = 0, position = -1;
	size_t i = 0;

	std::ifstream f("y2015/d01/input");
	while (!f.eof()) {
		const char c = f.get();
		if (c == '(') {
			floor++;
		} else if (c == ')') {
			floor--;
		}

		if (i++ && position == -1 && floor < 0) {
			position = i;
		}
	}

	strncpy(buf1, std::to_string(floor).c_str(), buflen);
	strncpy(buf2, std::to_string(position).c_str(), buflen);
}
