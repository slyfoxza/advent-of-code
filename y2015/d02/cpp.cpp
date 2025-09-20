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
#include <algorithm>
#include <cstring>
#include <fstream>

int max3(int a, int b, int c) {
	return std::max(a, std::max(b, c));
}

int min3(int a, int b, int c) {
	return std::min(a, std::min(b, c));
}

void cpp_y2015_d02(char* buf1, char* buf2, size_t buflen) {
	unsigned paper = 0, ribbon = 0;

	std::ifstream f("y2015/d02/input");
	while (true) {
		char c;
		int l, w, h;
		f >> l >> c >> w >> c >> h;
		if (f.eof() || f.fail()) break;

		int lw = l * w, wh = w * h, hl = h * l;
		paper += 2 * (lw + wh + hl) + min3(lw, wh, hl);
		ribbon += 2 * (l + w + h - max3(l, w, h)) + l * w * h;
	}

	strncpy(buf1, std::to_string(paper).c_str(), buflen);
	strncpy(buf2, std::to_string(ribbon).c_str(), buflen);
}
