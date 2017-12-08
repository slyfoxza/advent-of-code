#!/usr/bin/env perl6
use MONKEY-SEE-NO-EVAL; # The "I know what I'm doing" pragma
my $all_time_high = -Inf;
my %registers is default(0);

grammar G {
	rule TOP { <instruction> if <comparison> }
	proto rule instruction {*}
	      rule instruction:sym<deq> { <register> 'dec' <value> }
	      rule instruction:sym<inc> { <register> 'inc' <value> }
	rule comparison { <register> <cmpop> <value> }
	token cmpop { <[!=<>]>'='? }
	token register { \w+ }
	token value { '-'?\d+ }
}

class C {
	method TOP ($/) {
		if $<comparison>.made {
			%registers{$<instruction>.hash<register>} = $<instruction>.made;
			$all_time_high max= $<instruction>.made;
		}
	};

	method instruction:sym<deq> ($/) { make %registers{$<register>} - $<value> }
	method instruction:sym<inc> ($/) { make %registers{$<register>} + $<value> }

	method comparison ($/) { make EVAL "$(%registers{$<register>}) $($<cmpop>) $($<value>)" }
}

for $*IN.lines { die "Invalid input: $_" unless G.parse($_, actions => C) }
say %registers.values.max, ' ', $all_time_high
