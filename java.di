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
alias jboolean = ubyte;
alias jchar = ushort;
alias jshort = short;
alias jint = int;
alias jlong = long;

alias jfloat = float;
alias jdouble = double;

alias jsize = jint;

alias jobject = void*;
alias jclass = jobject;
alias jstring = jobject;
alias jthrowable = jobject;

alias jmethodID = void*;

enum JNI_OK = 0;

struct JNINativeInterface {
	void* reserved0, reserved1, reserved3, reserved4;

	void*[2] unused1; // GetVersion, DefineClass
	jclass function(JNIEnv*, const char*) FindClass;
	void*[8] unused2; // FromReflectedMethod -- AllocObject
	jthrowable function(JNIEnv*) ExceptionOccurred;
	void function(JNIEnv*) ExceptionDescribe;
	void*[11] unused3; // ExceptionClear -- AllocObject
	jobject function(JNIEnv*, jclass, jmethodID, ...) NewObject;
	void*[4] unused4; // NewObjectV -- IsInstanceOf
	jmethodID function(JNIEnv*, jclass, const char*, const char*) GetMethodID;
	jobject function(JNIEnv*, jobject, jmethodID, ...) CallObjectMethod;
	void*[134] unused5; // CallObjectMethodV -- GetStringUTFLength
	const char* function(JNIEnv*, jstring, jboolean*) GetStringUTFChars;
	void function(JNIEnv*, jstring, const char*) ReleaseStringUTFChars;
}

struct JNIEnv {
	JNINativeInterface *functions;
}

struct JNIInvokeInterface {
	void* reserved0, reserved1, reserved2;
	jint function(JavaVM*) DestroyJavaVM;
}

struct JavaVM {
	JNIInvokeInterface* functions;
}

struct JavaVMOption {
	const char* optionString;
	void* extraInfo;
}

struct JavaVMInitArgs {
	jint jniVersion;

	jint nOptions;
	JavaVMOption* options;
	jboolean ignoreUnrecognized;
}
