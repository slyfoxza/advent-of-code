#!/usr/bin/env perl6
my $accum = 0, my $i = 0, my $negind = Nil;
while defined $_ = $*IN.getc {
	given $_ {
		when '(' { $accum += 1; }
		when ')' { $accum -= 1; }
		default { next }
	};
	$i++;
	$negind = $i if $accum < 0 && !defined $negind
}
say $accum, ' ', $negind
# vim: set noet sw=4 ts=4:
