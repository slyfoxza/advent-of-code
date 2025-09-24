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

typedef struct {
	int x, y;
} Point;

typedef struct {
	Point key;
	bool present;
} HashTableEntry;

const size_t hashSize = 2639;

bool hash_table_add(HashTableEntry* hashTable, Point* point) {
	size_t index = ((point->x * 17) * 13 + (point->y * 17)) % hashSize;
	for (size_t n = 0, i = index; n < hashSize; i = (i + 1) % hashSize, n++) {
		if (hashTable[i].present) {
			if (memcmp(&hashTable[i].key, point, sizeof(Point)) == 0) {
				return true;
			}
			continue;
		}

		hashTable[i].present = true;
		hashTable[i].key = *point;
		return true;
	}

	return false;
}

size_t hash_table_size(HashTableEntry* hashTable) {
	size_t size = 0;
	for (size_t i = 0; i < hashSize; i++) {
		if (hashTable[i].present) {
			size++;
		}
	}
	return size;
}

void c_y2015_d03(char* buf1, char* buf2, size_t buflen) {
	FILE* f = fopen("y2015/d03/input", "r");
	if (f == NULL) {
		strncpy(buf1, "Error:", buflen);
		strerror_r(errno, buf2, buflen);
		return;
	}

	HashTableEntry* visited1 = calloc(hashSize, sizeof(HashTableEntry));
	HashTableEntry* visited2 = calloc(hashSize, sizeof(HashTableEntry));

	Point soloSanta = {0, 0};

	Point santa = {0, 0};
	Point robo = {0, 0};
	Point* current = &robo;

	hash_table_add(visited1, &soloSanta);
	hash_table_add(visited2, current);

	char buffer[4096];
	for (;;) {
		size_t nread = fread(buffer, 1, 4096, f);
		if (nread < 4096 && ferror(f)) {
			strncpy(buf1, "Error:", buflen);
			strerror_r(errno, buf2, buflen);
			free(visited1);
			free(visited2);
			fclose(f);
			return;
		}

		for (size_t j = 0; j < nread; j++) {
			current = current == &robo ? &santa : &robo;
			switch (buffer[j]) {
				case '^':
					soloSanta.y--;
					current->y--;
					break;
				case 'v':
					soloSanta.y++;
					current->y++;
					break;
				case '<':
					soloSanta.x--;
					current->x--;
					break;
				case '>':
					soloSanta.x++;
					current->x++;
					break;
			}

			if (!hash_table_add(visited1, &soloSanta) || !hash_table_add(visited2, current)) {
				strncpy(buf1, "Error:", buflen);
				strncpy(buf2, "Hash table too small", buflen);
				free(visited1);
				free(visited2);
				fclose(f);
				return;
			}
		}

		if (nread < 4096 && feof(f)) {
			fclose(f);
			break;
		}
	}

	snprintf(buf1, buflen, "%lu", hash_table_size(visited1));
	snprintf(buf2, buflen, "%lu", hash_table_size(visited2));

	free(visited1);
	free(visited2);
}
