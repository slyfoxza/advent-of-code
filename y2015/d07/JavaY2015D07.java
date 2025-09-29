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
package aoc.solutions;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.function.BinaryOperator;
import java.util.stream.Stream;

import aoc.AdventOfCode.Answers;
import aoc.AdventOfCode.Solution;

public class JavaY2015D07 implements Solution {
	private final Map<String, WireSource> wires = new HashMap<>();
	private final Map<String, Short> values = new HashMap<>();

	@Override
	public Answers solve() {
		try (final Stream<String> lines = Files.lines(Paths.get("y2015/d07/input"))) {
			lines.forEach((line) -> {
				int separatorIndex = line.lastIndexOf(" -> ");
				if (separatorIndex == -1) {
					throw new RuntimeException("Separator (\" -> \") not found");
				}

				String targetWire = line.substring(separatorIndex + 4);
				String sourceBits[] = line.substring(0, separatorIndex).split(" ");

				WireSource source;
				switch (sourceBits.length) {
					case 1:
						source = createOperand(sourceBits[0]);
						break;
					case 2:
						source = new NotWireSource(sourceBits[1]);
						break;
					case 3:
						BinaryOperator<Short> operation;
						switch (sourceBits[1]) {
							case "AND":
								operation = (a, b) -> (short)(a & b);
								break;
							case "OR":
								operation = (a, b) -> (short)(a | b);
								break;
							case "LSHIFT":
								operation = (a, b) -> (short)(a << b);
								break;
							case "RSHIFT":
								operation = (a, b) -> (short)(a >> b);
								break;
							default:
								throw new RuntimeException("Unhandled binary operation: " +
										sourceBits[1]);
						}
						source = new BinaryOpWireSource(
								createOperand(sourceBits[0]),
								createOperand(sourceBits[2]),
								operation
						);
						break;
					default:
						throw new RuntimeException("Unhandled sourceBits.length: " +
								sourceBits.length);
				}

				wires.put(targetWire, source);
			});
		} catch (final IOException e) {
			throw new UncheckedIOException(e);
		}

		final Short part1 = resolve("a");
		values.clear();
		wires.put("b", new ConstantSource(part1));
		final Short part2 = resolve("a");

		return new Answers(part1.toString(), part2.toString());
	}

	private WireSource createOperand(final String operand) {
		if (Character.isDigit(operand.charAt(0))) {
			return new ConstantSource(Short.valueOf(operand));
		} else {
			return new CopyWireSource(operand);
		}
	}

	/**
	 * Perform passes over `wires` until the targeted wire has an entry in `value`. Each pass adds
	 * values for each wire that has values for all of its inputs.
	 */
	private Short resolve(final String targetId) {
		while (!values.containsKey(targetId)) {
			for (final String id : wires.keySet()) {
				if (values.containsKey(id)){
					continue;
				}

				Short value;
				final var wire = wires.get(id);
				switch (wire) {
					case ConstantSource constant:
						values.put(id, constant.value());
						break;
					case CopyWireSource copy:
						value = values.get(copy.sourceId());
						if (value != null) {
							values.put(id, value);
						}
						break;
					case NotWireSource not:
						value = values.get(not.sourceId());
						if (value != null) {
							values.put(id, (short)~value);
						}
						break;
					case BinaryOpWireSource binaryOp:
						final var left = resolve(binaryOp.leftSource());
						final var right = resolve(binaryOp.rightSource());
						if (left != null && right != null) {
							values.put(id, binaryOp.operation.apply(left, right));
						}
						break;
					default:
						throw new RuntimeException("Unhandled wire type: " + wire.getClass());
				}
			}
		}

		return values.get(targetId);
	}

	private Short resolve(final WireSource operand) {
		switch (operand) {
			case ConstantSource constant:
				return constant.value();
			case CopyWireSource copy:
				return values.get(copy.sourceId());
			default:
				throw new RuntimeException("Unsupported operand type: " + operand.getClass());
		}
	}

	/** Marker interface. */
	private static interface WireSource {}

	private static record ConstantSource(Short value) implements WireSource {}

	private static record CopyWireSource(String sourceId) implements WireSource {}

	private static record NotWireSource(String sourceId) implements WireSource {}

	private static record BinaryOpWireSource(
			WireSource leftSource,
			WireSource rightSource,
			BinaryOperator<Short> operation
	) implements WireSource {}
}
