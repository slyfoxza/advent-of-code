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

void c_y2025_d01(char* buf1, char* buf2, size_t buflen) {
	FILE* f = fopen("y2025/d01/input", "r");
	if (f == NULL) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		return;
	}

	char* line = NULL;
	size_t n;
	int dial = 50;
	int zeroes = 0, clicks = 0;
	while (getline(&line, &n, f) != -1) {
		int value = atoi(line + 1);
		int sign = line[0] == 'L' ? -1 : 1;

		while (value > 100) {
			value -= 100;
			clicks++;
		}

		int dialStart = dial;
		dial = (dial + (sign * value)) % 100;
		if (dial < 0) dial += 100;

		if (dial == 0) zeroes++;

		if (dialStart != 0 && sign == -1 && value >= dialStart) {
			clicks++;
		} else if (sign == 1 && value >= (100 - dialStart)) {
			clicks++;
		}
	}

	free(line);
	if (ferror(f)) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		fclose(f);
		return;
	}
	fclose(f);

	snprintf(buf1, buflen, "%d", zeroes);
	snprintf(buf2, buflen, "%d", clicks);
}
