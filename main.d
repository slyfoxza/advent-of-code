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
import std.algorithm : map;
import std.array : split;
import std.conv : parse;
import std.datetime.stopwatch : StopWatch;
import std.format : format;
import std.getopt : defaultGetoptPrinter, getopt;
import std.range : enumerate;
import std.stdio : File, write;
import std.typecons : Nullable, tuple;

import aoc : Answers;
import registry : getSolutions, isSlow, registerDotNetSolutions, registerForkInterpreterSolutions,
	   registerInProcInterpreterSolutions, registerJvmSolutions;

void main(string[] args) {
	bool slow = false;

	auto helpInformation = getopt(
		args,
		"slow", &slow,
	);
	if (helpInformation.helpWanted) {
		defaultGetoptPrinter("Advent of Code", helpInformation.options);
		return;
	}

	foreach(year, day, expected; readAnswers()) {
		if (!slow && isSlow(year, day)) {
			continue;
		}

		write(format("%d-%02d", year, day));
		auto stopWatch = StopWatch();
		bool ranSolution = false;

		registerJvmSolutions(year, day);
		registerDotNetSolutions(year, day);
		registerInProcInterpreterSolutions(year, day);
		registerForkInterpreterSolutions(year, day);

		foreach (solution; getSolutions(year, day)) {
			ranSolution = true;
			immutable answers = solution.run(stopWatch);

			write(" ", solution.name);
			if (expected.isNull) {
				write("\xf0\x9f\x92\xa1 ", answers.part1, " ", answers.part2);
			} else if (expected.get() == answers) {
				write("\xe2\x9c\x85");
			} else {
				write("\xe2\x9d\x8c ", answers.part1, " ", answers.part2);
			}

			auto duration = stopWatch.peek.split!("msecs", "usecs");
			write(format("|%d.%03dms", duration.msecs, duration.usecs));
		}

		if (ranSolution) {
			write("\n");
		} else {
			write("\r");
		}
	}
}

/** Read details of each day from the `answers` file. */
auto readAnswers() {
	return File("answers", "r")
		.byLine
		.map!split
		.enumerate(1)
		.map!((line) {
			if (line.value.length != 4) {
				throw new Exception(format("L%d: invalid answer line", line.index));
			}

			/* Each line in the file looks like:
			 *     [year] [day] [part 1 answer] [part 2 answer]
			 * Either answer may be "?" where the correct answer is not yet known. */
			Nullable!Answers expected;
			if (line.value[2] != "?" && line.value[3] != "?") {
				expected = Answers(line.value[2].idup, line.value[3].idup);
			}
			return tuple(
					line.value[0].parse!ushort,
					line.value[1].parse!ubyte,
					expected
			);
		});
}
