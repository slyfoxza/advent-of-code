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

bool isVowel(char c) {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

void c_y2015_d05(char* buf1, char* buf2, size_t buflen) {
	FILE* f = fopen("y2015/d05/input", "r");
	if (f == NULL) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		return;
	}

	unsigned nice = 0, nice2 = 0;

	char* line = NULL;
	size_t n;
	ssize_t nread;
	while ((nread = getline(&line, &n, f)) != -1) {
		line[nread - 1] = '\0';

		unsigned vowels = 0;
		bool doubled = false;
		bool forbiddenPair = false;

		bool doublePair = false;
		bool sandwichPair = false;

		if (isVowel(line[0])) vowels++;
		for (int i = 0; i < nread - 1; i++) {
			char c0 = line[i];
			char c1 = line[i + 1];
			if ((c0 == 'a' || c0 == 'c' || c0 == 'p' || c0 == 'x') && c1 == c0 + 1) {
				forbiddenPair = true;
			}

			if (isVowel(c1)) vowels++;
			if (c0 == c1) doubled = true;

			if (c0 == line[i + 2]) {
				sandwichPair = true;
			}

			char needle[3] = {c0, c1, '\0'};
			if (!doublePair && strstr(&line[i + 2], needle) != NULL) {
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

	free(line);
	if (ferror(f)) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		fclose(f);
		return;
	}

	fclose(f);
	snprintf(buf1, buflen, "%d", nice);
	snprintf(buf2, buflen, "%d", nice2);
}
