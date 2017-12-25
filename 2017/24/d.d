import std.array;
import std.algorithm;
import std.conv;
import std.stdio;

int findBridge(T)(int cumStrength, int nextPort, T components) {
	int subStrength = cumStrength + nextPort;
	foreach(i, comp; components) {
		foreach(j; 0 .. 2) {
			if(comp[j] == nextPort) {
				int compStrength = findBridge(cumStrength + nextPort * 2, comp[j ^ 1], array(components).remove(i));
				if(compStrength > subStrength) {
					subStrength = compStrength;
				}
			}
		}
	}
	return subStrength;
}

void main() {
	auto components = stdin.byLine()
		.map!(a => a.split('/').map!(to!int).array.sort().array);
	writeln(findBridge(0, 0, array(components)));
}
