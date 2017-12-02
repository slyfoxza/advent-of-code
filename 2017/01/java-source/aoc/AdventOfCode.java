package aoc;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.IOException;

public class AdventOfCode {
	public static void main(final String[] arguments) throws IOException {
		final String input;
		try(final BufferedReader reader = new BufferedReader(new InputStreamReader(System.in))) {
			input = reader.readLine();
		}

		final int[] offsets = new int[] { 1, input.length() / 2 };
		final int[] accum = new int[] { 0, 0 };
		for(int i = 0; i < input.length(); ++i) {
			for(int j = 0; j < offsets.length; ++j) {
				if(input.charAt((i + offsets[j]) % input.length()) == input.charAt(i)) {
					accum[j] += Character.digit(input.charAt(i), 10);
				}
			}
		}

		System.out.print(accum[0]);
		System.out.print(" ");
		System.out.println(accum[1]);
	}
}

// vim: set noet sw=4 ts=4:
