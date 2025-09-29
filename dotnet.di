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
extern (C):
alias hostfxr_handle = void*;

enum hostfxr_delegate_type {
	hdt_com_activation,
	hdt_load_in_memory_assembly,
	hdt_winrt_activation,
	hdt_com_register,
	hdt_com_unregister,
	hdt_load_assembly_and_get_function_pointer,
	hdt_get_function_pointer,
	hdt_load_assembly,
	hdt_load_assembly_bytes
}

int get_hostfxr_path(char*, size_t*, void*);

int hostfxr_initialize_for_runtime_config(const char*, void*, hostfxr_handle*);
