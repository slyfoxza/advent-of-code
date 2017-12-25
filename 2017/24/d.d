import std.array;
import std.algorithm;
import std.conv;
import std.stdio;
import std.typecons;

alias Strlength = Tuple!(int, "strength", int, "length");

Strlength findBridge(T)(T components, bool function(Strlength, Strlength) pred, int length = 0, int cumStrength = 0, int nextPort = 0) {
	auto subStrength = Strlength(cumStrength + nextPort, length);
	foreach(i, comp; components) {
		foreach(j; 0 .. 2) {
			if(comp[j] == nextPort) {
				auto compStrength = findBridge(array(components).remove(i), pred, length + 1, cumStrength + nextPort * 2, comp[j ^ 1]);
				if(pred(compStrength, subStrength)) {
					subStrength = compStrength;
				}
			}
		}
	}
	return subStrength;
}

void main() {
	auto components = stdin.byLine()
		.map!(a => a.split('/').map!(to!int).array.sort().array)
		.array;
	writeln(
		findBridge(components, (a, b) => a.strength > b.strength).strength,
		' ',
		findBridge(components, (a, b) => a.length >= b.length && a.strength > b.strength).strength
	);
}
