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
import { Buffer } from 'node:buffer';
import { open } from 'node:fs/promises';

let floor = 0, position = -1, i = 0;

const buffer = Buffer.alloc(4096);
const f = await open('y2015/d01/input');
while (true) {
	const { bytesRead } = await f.read(buffer);
	if (bytesRead == 0) {
		break;
	}

	for (let j = 0; j < bytesRead; j++) {
		if (buffer[j] == 40) {
			floor++;
		} else if (buffer[j] == 41) {
			floor--;
		}

		if (i++ && position == -1 && floor < 0) {
			position = i;
		}
	}
}

console.log(floor, position);
