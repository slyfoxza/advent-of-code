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
#include <unordered_set>

struct Point {
	int x, y;

	bool operator==(const Point& p) const noexcept {
		return x == p.x && y == p.y;
	}
};

template<>
struct std::hash<Point> {
	std::size_t operator()(const Point& p) const noexcept {
		return (p.x * 13) * 17 + (p.y * 13);
	}
};

void cpp_y2015_d03(char* buf1, char* buf2, size_t buflen) {
	std::unordered_set<Point> visited1, visited2;

	Point soloSanta{0, 0};

	Point santa{0, 0};
	Point robo{0, 0};
	Point* current = &robo;

	visited1.insert(soloSanta);
	visited2.insert(*current);

	std::ifstream f("y2015/d03/input");
	while (!f.eof()) {
		current = current == &robo ? &santa : &robo;

		const char c = f.get();
		switch (c) {
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

		visited1.insert(soloSanta);
		visited2.insert(*current);
	}

	strncpy(buf1, std::to_string(visited1.size()).c_str(), buflen);
	strncpy(buf2, std::to_string(visited2.size()).c_str(), buflen);
}
