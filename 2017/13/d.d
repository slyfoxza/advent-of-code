import std.algorithm.comparison : max;
import std.algorithm.iteration : splitter;
import std.conv : to;
import std.stdio : readln, writefln;
import std.string : strip;

void main() {
	char[] buf;
	int[int] firewall;
	int maxIndex = -1;
	while(readln(buf)) {
		auto s = buf.strip().splitter(": ");
		auto i = to!int(s.front); s.popFront();
		firewall[i] = to!int(s.front);
		maxIndex = max(maxIndex, i);
	}

	int severity = 0;
	for(int i = 0; i <= maxIndex; ++i) {
		if(!(i in firewall)) continue;
		if(i % (2 * firewall[i] - 2) == 0) {
			severity += i * firewall[i];
		}
	}

	int delay = 0;
	delayLoop:
	for(;; ++delay) {
		for(int i = 0; i <= maxIndex; ++i) {
			if(!(i in firewall)) continue;
			if((delay + i) % (2 * firewall[i] - 2) == 0) {
				continue delayLoop;
			}
		}
		break;
	}

	writefln("%d %d", severity, delay);
}
