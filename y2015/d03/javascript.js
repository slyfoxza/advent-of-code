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

const soloSantaPos = { x: 0, y: 0 };

const santaPos = { x: 0, y: 0 };
const roboPos = { x: 0, y: 0 };
let current = roboPos;

const visited1 = new Set(["0,0"]);
const visited2 = new Set(["0,0"]);

const buffer = Buffer.alloc(4096);
const f = await open('y2015/d03/input');
while (true) {
	const { bytesRead } = await f.read(buffer);
	if (bytesRead === 0) {
		break;
	}

	for (let i = 0; i < bytesRead; i++) {
		current = current === roboPos ? santaPos : roboPos;

		let dx = 0, dy = 0;
		switch (buffer[i]) {
			case 94: // '^'
				dy = -1;
				break;
			case 118: // 'v'
				dy = 1;
				break;
			case 60: // '<'
				dx = -1;
				break;
			case 62: // '>'
				dx = 1;
				break;
		}

		soloSantaPos.y += dy;
		soloSantaPos.x += dx;
		current.y += dy;
		current.x += dx;

		visited1.add(`${soloSantaPos.x},${soloSantaPos.y}`);
		visited2.add(`${current.x},${current.y}`);
	}
}

f.close();
console.log(visited1.size, visited2.size);
