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
import { hash } from 'node:crypto';
import { readFile } from 'node:fs/promises';

const fileContents = await readFile('y2015/d04/input', { encoding: 'utf8' });
const secretKey = fileContents.trimEnd();

let part1 = 0;
for (let i = 1; ; i++) {
	const h = hash('md5', secretKey + i.toString(), 'buffer')
	if (h[0] == 0 && h[1] == 0) {
		if (part1 == 0 && (h[2] & 0xF0) == 0) {
			part1 = i;
		}
		if (h[2] == 0) {
			console.log(part1, i);
			break;
		}
	}
}
