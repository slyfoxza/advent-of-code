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

int removeRolls(char* grid, size_t w, size_t h) {
	int removed = 0;
	for (int yi = 0; yi < h; ++yi) {
		int y = yi * h;
		for (int x = 0; x < w; ++x) {
			if (grid[y + x] != '@') continue;

			int adjCount = 0;
			for (int dx = -1; dx <= 1; dx++) {
				for (int dy = -1; dy <= 1; dy++) {
					if (dx == 0 && dy == 0) continue;
					int x1 = x + dx;
					int yi1 = (yi + dy);
					if (x1 < 0 || x1 >= w || yi1 < 0 || yi1 >= h) continue;
					int y1 = yi1 * h;

					if (grid[y1 + x1] != '.') adjCount++;
					if (adjCount >= 4) break;
				}
				if (adjCount >= 4) break;
			}

			if (adjCount < 4) {
				grid[y + x] = 'x';
				removed++;
			}
		}
	}

	for (int yi = 0; yi < h; ++yi) {
		int y = yi * h;
		for (int x = 0; x < w; ++x) {
			if (grid[y + x] == 'x') grid[y + x] = '.';
		}
	}

	return removed;
}

void c_y2025_d04(char* buf1, char* buf2, size_t buflen) {
	FILE* f = fopen("y2025/d04/input", "r");
	if (f == NULL) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		return;
	}

	size_t bufsize = 0;
	char* buffer = NULL;
	char* pbuffer = NULL;

	char* line = NULL;
	size_t n;
	size_t w = 0, h = 0;
	while (getline(&line, &n, f) != -1) {
		size_t len = strlen(line);
		line[--len] = '\0'; // nuke the newline

		if (w == 0) w = len;
		++h;

		if (buffer == NULL || len > bufsize - (pbuffer - buffer)) {
			bufsize += 1024 * 1024;
			buffer = realloc(buffer, bufsize);

			if (pbuffer == NULL) pbuffer = buffer;
		}

		pbuffer = memcpy(pbuffer, line, len);
		pbuffer += len;
	}

	int accessible = removeRolls(buffer, w, h);

	int totalRemoved = accessible;
	int removed;
	while ((removed = removeRolls(buffer, w, h)) != 0) {
		totalRemoved += removed;
	}

	free(line);
	if (ferror(f)) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		fclose(f);
		return;
	}

	fclose(f);
	snprintf(buf1, buflen, "%d", accessible);
	snprintf(buf2, buflen, "%d", totalRemoved);
}
