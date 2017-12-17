import std.algorithm;
import std.array;
import std.stdio;

void main() {
	int skip;
	stdin.readf!"%d"(skip);
	int[] ring = [0];
	ring.reserve(2017);
	uint i = 0;
	for(int n = 1; n <= 2017; ++n) {
		i = (i + skip) % ring.length;
		ring.insertInPlace(i + 1, n);
		i = i + 1;
	}
	writeln(ring[i + 1]);
}
