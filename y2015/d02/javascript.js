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
import { open } from 'node:fs/promises';

let paper = 0, ribbon = 0;

const f = await open('y2015/d02/input');
for await (const line of f.readLines()) {
	const [l, w, h] = line.split('x').map(Number);
	const lw = l * w, wh = w * h, hl = h * l;
	paper += 2 * (lw + wh + hl) + Math.min(lw, wh, hl);
	ribbon += 2 * (l + w + h - Math.max(l, w, h)) + l * w * h;
}

console.log(paper, ribbon);
