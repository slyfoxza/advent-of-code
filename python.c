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
#include <Python.h>

/* Bit of a hack to not have to specify the exact PyConfig structure in D: allocate the structure
 * on the heap, which lets us pass it around as a pointer and ignore the actual struct layout. */
PyConfig* aoc_alloc_pyconfig() {
	return malloc(sizeof(PyConfig));
}

// Wrap Python functions implemented as macros

int aoc_pylong_check(PyObject* p) {
	return PyLong_Check(p);
}

int aoc_pytuple_check(PyObject* p) {
	return PyTuple_Check(p);
}

int aoc_pyunicode_check(PyObject* p) {
	return PyUnicode_Check(p);
}
