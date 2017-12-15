import std.algorithm;
import std.array;
import std.container;
import std.conv;
import std.stdio;

void main() {
	const DIV = 2147483647;

	auto generators = stdin.byLine()
		.map!(a => a.split()[$-1].to!long)
		.array();
	int[2] matches = [0, 0];
	auto gen0 = DList!long();
	auto gen1 = DList!long();
	int j = 0;
	for(int i = 0; i < 40_000_000 && j < 5_000_000; ++i) {
		generators[0] *= 16807; generators[1] *= 48271;
		generators[0] %= DIV;   generators[1] %= DIV;
		if((generators[0] & 0xFFFF) == (generators[1] & 0xFFFF)) {
			matches[0]++;
		}

		if((generators[0] & 3) == 0) {
			gen0.insertBack(generators[0]);
		}
		if((generators[1] & 7) == 0) {
			gen1.insertBack(generators[1]);
		}

		if(!gen0.empty() && !gen1.empty()) {
			if((gen0.front() & 0xFFFF) == (gen1.front() & 0xFFFF)) {
				matches[1]++;
			}
			gen0.removeFront();
			gen1.removeFront();
			++j;
		}
	}
	writefln("%d %d", matches[0], matches[1]);
}
