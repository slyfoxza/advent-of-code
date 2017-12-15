import std.algorithm;
import std.array;
import std.conv;
import std.stdio;

void main() {
	const DIV = 2147483647;

	auto generators = stdin.byLine()
		.map!(a => a.split()[$-1].to!long)
		.array();
	int matches = 0;
	for(int i = 0; i < 40_000_000; ++i) {
		generators[0] *= 16807; generators[1] *= 48271;
		generators[0] %= DIV;   generators[1] %= DIV;
		if((generators[0] & 0xFFFF) == (generators[1] & 0xFFFF)) {
			matches++;
		}
	}
	writefln("%d", matches);
}
