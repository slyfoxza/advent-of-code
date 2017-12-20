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
		ring.insertInPlace(i++, n);
	}
	writef("%d ", ring[i]);

	i = 0;
	int r1 = -1;
	for(int n = 1; n <= 50_000_000; ++n) {
		i = (i + skip) % n;
		if(i++ == 0) {
			r1 = n;
		}
	}
	writeln(r1);
}
