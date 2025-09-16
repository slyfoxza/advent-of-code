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
#include <errno.h>
#include <stdio.h>
#include <string.h>

void c_y2015_d01(char* buf1, char* buf2, size_t buflen) {
	FILE* f = fopen("y2015/d01/input", "r");
	if (f == NULL) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		return;
	}

	int floor = 0, position = -1;
	size_t i = 0;
	char buffer[4096];
	for (;;) {
		size_t nread = fread(buffer, 1, 4096, f);
		if (nread < 4096 && ferror(f)) {
			strncpy(buf1, "Error:", buflen);
			strerror_r(errno, buf2, buflen);
			fclose(f);
			return;
		}

		for (size_t j = 0; j < nread; j++) {
			if (buffer[j] == '(') {
				floor++;
			} else if (buffer[j] == ')') {
				floor--;
			}

			if (i++ && position == -1 && floor < 0) {
				position = i;
			}
		}

		if (nread < 4096 && feof(f)) {
			fclose(f);
			break;
		}
	}

	snprintf(buf1, buflen, "%d", floor);
	snprintf(buf2, buflen, "%d", position);
}
