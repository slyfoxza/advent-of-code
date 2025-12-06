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
export default function (plop) {
	plop.setHelper('equals', (a, b) => a == b);
	plop.setHelper('zeroPadStart', (value, length) => value.toString().padStart(length, "0"));

	const commonPrompts = [
		{ type: 'number', name: 'year' },
		{ type: 'number', name: 'day' },
	];
	const directory = 'y{{year}}/d{{zeroPadStart day 2}}';

	plop.setGenerator('c', {
		prompts: [
			...commonPrompts,
			{
				type: 'list',
				name: 'readMode',
				choices: ['line'],
			},
		],
		actions: [
			{
				type: 'add',
				path: `${directory}/c.c`,
				templateFile: 'plop-templates/c.hbs',
			},
		],
	});
	plop.setGenerator('csharp', {
		prompts: [
			...commonPrompts,
			{
				type: 'list',
				name: 'readMode',
				choices: ['line'],
			},
		],
		actions: [
			{
				type: 'add',
				path: `${directory}/csharp.cs`,
				templateFile: 'plop-templates/csharp.hbs',
			},
		],
	});
	plop.setGenerator('d', {
		prompts: [
			...commonPrompts,
			{
				type: 'list',
				name: 'readMode',
				choices: ['line'],
			},
		],
		actions: [
			{
				type: 'add',
				path: `${directory}/d.d`,
				templateFile: 'plop-templates/d.hbs',
			},
		],
	});
	plop.setGenerator('python', {
		prompts: [
			...commonPrompts,
			{
				type: 'list',
				name: 'readMode',
				choices: ['chunk', 'line', 'single'],
			},
		],
		actions: [
			{
				type: 'add',
				path: `${directory}/python.py`,
				templateFile: 'plop-templates/python.hbs',
			},
		],
	});
}
