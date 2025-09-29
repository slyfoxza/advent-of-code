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
import std.algorithm : canFind;
import std.conv : to;
import std.datetime.stopwatch : StopWatch;
import std.file : exists, getcwd, isFile, read;
import std.format : format;
import std.process : execute;
import std.string : fromStringz, indexOf, stripRight, toStringz;
import std.typecons : Tuple, tuple;
import std.zip : ZipArchive;
import core.stdc.stdlib : free, malloc;
import core.sys.posix.dlfcn : RTLD_LAZY, dlopen, dlsym;

import aoc : Answers;
import dotnet;
import guile;
import java;
import lua;
import python;

private alias YearDay = Tuple!(ushort, "year", ubyte, "day");
private Solution[][YearDay] registry;

immutable SlowYearDays = [
	YearDay(2015, 4),
];

bool isSlow(ushort year, ubyte day) {
	return SlowYearDays.canFind(YearDay(year, day));
}

void register(ushort year, ubyte day, Solution[] solutions...) {
	immutable yearDay = YearDay(year, day);
	registry.require(yearDay, []);
	registry[yearDay] ~= solutions;
}

immutable JvmJarPath = "builddir/gradle/libs/advent-of-code.jar";
void registerJvmSolutions(ushort year, ubyte day) {
	immutable yearDay = YearDay(year, day);
	registry.require(yearDay, []);

	immutable needle = format("aoc/solutions/JavaY%dD%02d.class", year, day);
	foreach (name, zipMember; new ZipArchive(read(JvmJarPath)).directory) {
		if (name == needle) {
			registry[yearDay] ~= new JavaSolution(year, day);
			break;
		}
	}
}

void registerDotNetSolutions(ushort year, ubyte day) {
	immutable yearDay = YearDay(year, day);
	registry.require(yearDay, []);

	immutable func = DotNetSolution.getFunctionPointer(year, day);
	if (func != null) {
		registry[yearDay] ~= new DotNetSolution(func);
	}
}

void registerInProcInterpreterSolutions(ushort year, ubyte day) {
	immutable yearDay = YearDay(year, day);
	registry.require(yearDay, []);

	immutable dir = format("y%d/d%02d/", year, day);
	string filename;

	filename = dir ~ "guile.scm";
	if (filename.exists && filename.isFile) {
		registry[yearDay] ~= new GuileSolution(filename);
	}

	filename = dir ~ "lua.lua";
	if (filename.exists && filename.isFile) {
		registry[yearDay] ~= new LuaSolution(filename);
	}

	filename = dir ~ "python.py";
	if (filename.exists && filename.isFile) {
		registry[yearDay] ~= new PythonSolution(year, day);
	}
}

void registerForkInterpreterSolutions(ushort year, ubyte day) {
	immutable yearDay = YearDay(year, day);
	registry.require(yearDay, []);

	immutable dir = format("y%d/d%02d/", year, day);
	immutable specs = [
		tuple("JS", "javascript.js", "node"),
		// Just for kicks: the overhead of a child process
		// tuple("Python", "python.py", "python3"),
		tuple("Ruby", "ruby.rb", "ruby"),
	];
	foreach(spec; specs) {
		immutable filename = dir ~ spec[1];
		if (filename.exists && filename.isFile) {
			registry[yearDay] ~= new ForkInterpreterSolution(spec[0], filename, spec[2]);
		}
	}
}

Solution[] getSolutions(ushort year, ubyte day) {
	return registry.get(YearDay(year, day), []);
}

private class Solution {
	string name;

	this(string name) {
		this.name = name;
	}

	abstract Answers run(ref StopWatch sw);
}

class DSolution : Solution {
	private alias Function = Answers function();
	Function func;

	this(Function func, string name = "D") {
		super(name);
		this.func = func;
	}

	override Answers run(ref StopWatch sw) {
		sw.restart();
		scope(exit) sw.stop();
		return func();
	}
}

/**
 * A solution that will pass pre-allocated buffers to the implementation in which the answers should
 * be written.
 *
 * To be used for languages like C and C++, but especially a language like Rust where the ownership
 * model makes it difficult to return and manage Rust-allocated strings to the D calling code
 * without leaking memory.
 *
 * Buffers are allocated statically, so they are shared across invocations of solutions inheriting
 * from this class, avoiding many repeated small memory allocations and releases. For convenience,
 * both buffers will have an equal size.
 */
private class BufPassSolution(string Linkage, string Name) : Solution {
	protected immutable size_t bufferSize = 64;
	protected static char* buffer1, buffer2;

	mixin("alias Function = extern (" ~ Linkage ~ ") void function(char*, char*, size_t);");
	private Function func;

	shared static this()
	out(; buffer1 != null && buffer2 != null, "buffer allocation failure")
	{
		buffer1 = cast(char*) malloc(64);
		buffer2 = cast(char*) malloc(64);
	}

	this(Function func, string name = Name) {
		super(name);
		this.func = func;
	}

	override Answers run(ref StopWatch sw) {
		/* Zero out the first character of each buffer -- no cheating by cribbing on a previous
		 * solution! */
		*buffer1 = *buffer2 = '\0';
		sw.restart();
		func(buffer1, buffer2, bufferSize);
		sw.stop();
		// Ensure the buffers are null-terminated
		buffer1[bufferSize - 1] = buffer2[bufferSize - 1] = '\0';
		return Answers(buffer1.fromStringz.idup, buffer2.fromStringz.idup);
	}

	shared static ~this() {
		free(buffer1);
		free(buffer2);
	}
}

alias CSolution = BufPassSolution!("C", "C");
mixin template CExtern(string Symbol) {
	mixin("extern (C) void " ~ Symbol ~ "(char*, char*, size_t);");
}

alias CppSolution = BufPassSolution!("C++", "C++");
mixin template CppExtern(string Symbol) {
	mixin("extern (C++) void " ~ Symbol ~ "(char*, char*, size_t);");
}

alias RustSolution = BufPassSolution!("C", "Rust");

class GuileSolution : Solution {
	private string filename;

	this(string filename, string name = "Guile") {
		super(name);
		this.filename = filename;
	}

	private alias GuileData = Tuple!(immutable char*, "filename", StopWatch*, "sw", Answers*, "answers");

	override Answers run(ref StopWatch sw) {
		Answers answers;
		auto guileData = GuileData(filename.toStringz, &sw, &answers);
		scm_with_guile(&runGuile, cast(void*) &guileData);
		return answers;
	}

	private static extern (C) void* runGuile(void* ptr) {
		GuileData* guileData = cast(GuileData*) ptr;
		(*guileData.sw).restart();
		auto result = scm_c_primitive_load(guileData.filename);
		guileData.sw.stop();

		guileData.answers.part1 = answerToString(scm_c_value_ref(result, 0));
		if (scm_c_nvalues(result) >= 2) {
			guileData.answers.part2 = answerToString(scm_c_value_ref(result, 1));
		} else {
			guileData.answers.part2 = "0";
		}

		return null;
	}

	private static string answerToString(SCM value) {
		scm_dynwind_begin(0);
		scope(exit) scm_dynwind_end();

		char* stringz;
		if (value.scm_is_number) {
			auto scmString = scm_number_to_string(value, scm_from_int32(10));
			stringz = scm_to_utf8_stringn(scmString, null);
		} else {
			stringz = scm_to_utf8_stringn(value, null);
		}
		scope(exit) scm_dynwind_free(stringz);
		return stringz.fromStringz.dup;
	}
}

class LuaSolution : Solution {
	private static Lua lua = null;

	private string filename;

	shared static this() {
		lua = luaL_newstate();
		luaL_openlibs(lua);
	}

	this(string filename, string name = "Lua") {
		super(name);
		this.filename = filename;
	}

	override Answers run(ref StopWatch sw) {
		// Restore the Lua stack to its original size when exiting this scope
		auto originalTop = lua_gettop(lua);
		scope(exit) lua_settop(lua, originalTop);

		luaL_loadfilex(lua, filename.toStringz, null);
		if (lua_type(lua, -1) != LUA_TFUNCTION) {
			if (lua_isstring(lua, -1)) {
				return Answers("Lua load error", lua_tolstring(lua, -1, null).fromStringz.idup);
			} else {
				return Answers("Lua load error", "Unknown error");
			}
		}

		sw.restart();
		immutable result = lua_pcallk(lua, 0, 2, 0, 0, null);
		sw.stop();

		if (result == LUA_OK) {
			// The two values at the top of the stack will be the return values
			return Answers(answerToString(-2), answerToString(-1));
		} else {
			return Answers("-1", "-1");
		}
	}

	private string answerToString(int index) {
		if (lua_isstring(lua, index)) {
			return lua_tolstring(lua, index, null).fromStringz.idup;
		} else if (lua_isinteger(lua, index)) {
			immutable value = lua_tointegerx(lua, index, null);
			return value.to!string;
		} else {
			throw new Error("Unhandled Lua value type");
		}
	}

	shared static ~this() {
		if (lua != null) {
			lua_close(lua);
		}
	}
}

class PythonSolution : Solution {
	private string funcName;
	private string modulePath;

	shared static this() {
		auto pyConfig = aoc_alloc_pyconfig();
		PyConfig_InitIsolatedConfig(pyConfig);
		Py_InitializeFromConfig(pyConfig);
		// TODO: Should check the PyStatus here
		PyConfig_Clear(pyConfig);
		free(pyConfig);

		auto sysPath = PySys_GetObject("path");
		auto path = PyUnicode_FromString(getcwd().toStringz);
		PyList_Append(sysPath, path);
		Py_DecRef(path);
	}

	this(ushort year, ubyte day, string name = "Python") {
		super(name);
		funcName = format("python_y%d_d%02d", year, day);
		modulePath = format("y%d.d%02d.python", year, day);
	}

	override Answers run(ref StopWatch sw) {
		auto pyMod = PyImport_ImportModule(modulePath.toStringz);
		throwIfError();
		scope (exit) Py_DecRef(pyMod);

		auto pyFunc = PyObject_GetAttrString(pyMod, funcName.toStringz);
		throwIfError();
		scope (exit) Py_DecRef(pyFunc);

		sw.restart();
		auto pyResult = PyObject_CallNoArgs(pyFunc);
		sw.stop();
		throwIfError();
		scope (exit) Py_DecRef(pyResult);

		if (!aoc_pytuple_check(pyResult)) {
			throw new Exception("Python result was not a tuple");
		} else if (PyTuple_Size(pyResult) != 2) {
			throw new Exception("Python result was not 2-element tuple");
		}

		return Answers(answerToString(pyResult, 0), answerToString(pyResult, 1));
	}

	private string answerToString(PyObject tuple, int index) {
		auto pyObject = PyTuple_GetItem(tuple, index);
		if (aoc_pylong_check(pyObject)) {
			long value = PyLong_AsLong(pyObject);
			return value.to!string;
		} else {
			throw new Exception("Unhandled result type");
		}
	}

	private void throwIfError() {
		if (auto pyError = PyErr_GetRaisedException()) {
			scope(exit) Py_DecRef(pyError);

			auto errorArgs = PyException_GetArgs(pyError);
			scope(exit) Py_DecRef(errorArgs);

			if (!aoc_pytuple_check(errorArgs)) {
				throw new Exception("Unknown Python error");
			}

			auto errorArgsSize = PyTuple_Size(errorArgs);
			if (errorArgsSize < 1) {
				throw new Exception("Unknown Python error");
			}

			auto maybeMessage = PyTuple_GetItem(errorArgs, 0);
			if (!aoc_pyunicode_check(maybeMessage)) {
				throw new Exception("Unknown Python error");
			}

			throw new Exception(PyUnicode_AsUTF8(maybeMessage).fromStringz.idup);
		}
	}

	shared static ~this() {
		Py_Finalize();
	}
}

class JavaSolution : Solution {
	private static JNIEnv* jni;
	private static JavaVM* jvm;

	private static jmethodID solveMethodId;
	private static jmethodID answerPart1MethodId, answerPart2MethodId;

	shared static this() {
		auto libJava = dlopen("/usr/lib/jvm/jre-25/lib/server/libjvm.so", RTLD_LAZY);
		auto JNI_GetDefaultJavaVMInitArgs = cast(jint function(void*)) libJava.dlsym("JNI_GetDefaultJavaVMInitArgs");
		JavaVMInitArgs jvmInitArgs;
		jvmInitArgs.jniVersion = 0x00180000; // JNI_VERSION_24
		jvmInitArgs.nOptions = 1;
		immutable classPathArg = "-Djava.class.path=" ~ JvmJarPath;
		JavaVMOption[] jvmOptions = [{classPathArg.toStringz}];
		jvmInitArgs.options = &jvmOptions[0];

		auto JNI_CreateJavaVM = cast(jint function(JavaVM**, JNIEnv**, void*)) libJava.dlsym("JNI_CreateJavaVM");
		if (auto result = JNI_CreateJavaVM(&jvm, &jni, &jvmInitArgs) != JNI_OK) {
			throw new Exception(format("JNI_CreateJavaVM: %d", result));
		}

		auto solutionInterface = jni.functions.FindClass(jni, "aoc/AdventOfCode$Solution");
		solveMethodId = jni.functions.GetMethodID(jni, solutionInterface, "solve", "()Laoc/AdventOfCode$Answers;");

		auto answersClass = jni.functions.FindClass(jni, "aoc/AdventOfCode$Answers");
		answerPart1MethodId = jni.functions.GetMethodID(jni, answersClass, "part1", "()Ljava/lang/String;");
		answerPart2MethodId = jni.functions.GetMethodID(jni, answersClass, "part2", "()Ljava/lang/String;");
	}

	private string javaClassFqname;

	this(ushort year, ubyte day) {
		super("Java");
		javaClassFqname = format("aoc/solutions/JavaY%dD%02d", year, day);
	}

	override Answers run(ref StopWatch sw) {
		auto javaClass = jni.functions.FindClass(jni, javaClassFqname.toStringz);
		auto javaCtor = jni.functions.GetMethodID(jni, javaClass, "<init>", "()V");
		auto javaObject = jni.functions.NewObject(jni, javaClass, javaCtor);

		sw.restart();
		auto javaResult = jni.functions.CallObjectMethod(jni, javaObject, solveMethodId);
		sw.stop();

		Answers answers;

		auto javaString = jni.functions.CallObjectMethod(jni, javaResult, answerPart1MethodId);
		auto utfChars = jni.functions.GetStringUTFChars(jni, javaString, null);
		answers.part1 = utfChars.fromStringz.idup;
		jni.functions.ReleaseStringUTFChars(jni, javaString, utfChars);

		javaString = jni.functions.CallObjectMethod(jni, javaResult, answerPart2MethodId);
		utfChars = jni.functions.GetStringUTFChars(jni, javaString, null);
		answers.part2 = utfChars.fromStringz.idup;
		jni.functions.ReleaseStringUTFChars(jni, javaString, utfChars);

		return answers;
	}

	shared static ~this() {
		jvm.functions.DestroyJavaVM(jvm);
	}
}


class DotNetSolution : BufPassSolution!("C", "C#") {
	private static int function(const char*, const char*, const char*, const char*, void*, void**)
		load_assembly_and_get_function_pointer;

	shared static this() {
		char[256] path;
		size_t size = 256;
		get_hostfxr_path(cast(char*) path, &size, null);
		auto libHostfxr = dlopen(cast(char*) path, RTLD_LAZY);

		hostfxr_handle hostfxr;
		auto hostfxr_initialize_for_runtime_config =
			cast(int function(const char*, void*, hostfxr_handle*))
			libHostfxr.dlsym("hostfxr_initialize_for_runtime_config");
		auto result = hostfxr_initialize_for_runtime_config(
				"builddir/dotnet/bin/Debug/net9.0/advent-of-code.runtimeconfig.json",
				null, &hostfxr);

		auto hostfxr_get_runtime_delegate =
			cast(int function(hostfxr_handle, int, void**))
			libHostfxr.dlsym("hostfxr_get_runtime_delegate");
		hostfxr_get_runtime_delegate(
				hostfxr,
				hostfxr_delegate_type.hdt_load_assembly_and_get_function_pointer,
				cast(void**)&load_assembly_and_get_function_pointer
		);
	}

	this(Function func) {
		super(func);
	}

	static Function getFunctionPointer(ushort year, ubyte day) {
		Function func;
		auto result = load_assembly_and_get_function_pointer(
				"builddir/dotnet/bin/Debug/net9.0/advent-of-code.dll",
				format("CSharpY%dD%02d, advent-of-code", year, day).toStringz,
				"Solve",
				cast(const char*) -1, // UNMANAGEDCALLERSONLY_METHOD
				null,
				cast(void**) &func
		);
		if (result != 0) {
			return null;
		} else {
			return func;
		}
	}
}

class ForkInterpreterSolution : Solution {
	private string filename;
	private string interpreter;

	this(string name, string filename, string interpreter) {
		super(name);
		this.filename = filename;
		this.interpreter = interpreter;
	}

	override Answers run(ref StopWatch sw) {
		sw.restart();
		immutable execResult = execute([interpreter, filename]);
		sw.stop();

		immutable output = execResult.output.stripRight;
		if (execResult.status == 0) {
			immutable i = output.indexOf(' ');
			return Answers(output[0 .. i], output[i+1 .. $]);
		} else {
			return Answers(execResult.status.to!string, output);
		}
	}
}

private void restart(ref StopWatch sw) {
	if (sw.running) sw.stop();
	sw.reset();
	sw.start();
}

