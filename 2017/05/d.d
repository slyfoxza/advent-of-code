import std.array : appender;
import std.conv : to;
import std.stdio : readln, writefln;
import std.string : strip;

void main() {
	char[] buf;
	auto appender = appender!(int[]);
	appender.reserve(1093);
	while(readln(buf)) appender.put(to!int(strip(buf)));
	auto offsets1 = appender.data;
	int[] offsets2;
	offsets2.length = offsets1.length;
	offsets2[] = offsets1[];

	int[2] steps = [0, 0];
	for(int i = 0; (i >= 0) && (i < offsets1.length); ++steps[0]) {
		i += offsets1[i]++;
	}

	for(int i = 0; (i >= 0) && (i < offsets2.length); ++steps[1]) {
		int tmp = offsets2[i];
		offsets2[i] += (tmp >= 3) ? -1 : 1;
		i += tmp;
	}

	writefln("%d %d", steps[0], steps[1]);
}
