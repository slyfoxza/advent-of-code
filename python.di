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
alias Py_ssize_t = ptrdiff_t;
alias PyConfig = void*;
alias PyObject = void*;

struct PyStatus {
	const char* func;
	const char* err_msg;
	int exitcode;
}

extern (C):
PyConfig aoc_alloc_pyconfig();
int aoc_pylong_check(PyObject);
int aoc_pytuple_check(PyObject);
int aoc_pyunicode_check(PyObject);

void Py_DecRef(PyObject);
void Py_Finalize();
PyStatus Py_InitializeFromConfig(PyConfig);

void PyConfig_Clear(PyConfig);
void PyConfig_InitIsolatedConfig(PyConfig);

PyObject PyErr_GetRaisedException();

PyObject PyException_GetArgs(PyObject);

PyObject PyImport_ImportModule(const char*);

int PyList_Append(PyObject, PyObject);

long PyLong_AsLong(PyObject);

PyObject PyObject_CallNoArgs(PyObject);
PyObject PyObject_GetAttrString(PyObject, const char*);

PyObject PySys_GetObject(const char*);

PyObject PyTuple_GetItem(PyObject, Py_ssize_t);
Py_ssize_t PyTuple_Size(PyObject);

char* PyUnicode_AsUTF8(PyObject);
PyObject PyUnicode_FromString(const char*);
