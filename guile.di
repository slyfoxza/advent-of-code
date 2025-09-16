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
alias SCM = void*;

extern (C):
SCM scm_c_primitive_load(const char*);
size_t scm_c_nvalues(SCM);
SCM scm_c_value_ref(SCM, size_t);
void scm_dynwind_begin(int);
void scm_dynwind_end();
void scm_dynwind_free(void*);
SCM scm_from_int32(int);
int scm_is_number(SCM);
SCM scm_number_to_string(SCM, SCM);
char* scm_to_utf8_stringn(SCM, size_t*);
void* scm_with_guile(void* function(void*), void*);
