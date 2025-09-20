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
#include <stdlib.h>
#include <string.h>

int min3(int a, int b, int c) {
	int m = a < b ? a : b;
	return m < c ? m : c;
}

void c_y2015_d02(char* buf1, char* buf2, size_t buflen) {
	FILE* f = fopen("y2015/d02/input", "r");
	if (f == NULL) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		return;
	}

	unsigned paper = 0, ribbon = 0;
	char* line = NULL;
	size_t n;
	while (getline(&line, &n, f) != -1) {
		char* x1 = strchr(line, 'x');
		char* x2 = strchr(x1 + 1, 'x');
		*x1 = *x2 = '\0';

		int l = atoi(line), w = atoi(x1 + 1), h = atoi(x2 + 1);
		int lw = l * w, wh = w * h, hl = h * l;
		paper += 2 * (lw + wh + hl) + min3(lw, wh, hl);
		ribbon += 2 * (l + w + h + min3(-l, -w, -h)) + l * w * h;
	}

	free(line);
	if (ferror(f)) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		fclose(f);
		return;
	}

	snprintf(buf1, buflen, "%d", paper);
	snprintf(buf2, buflen, "%d", ribbon);
}
