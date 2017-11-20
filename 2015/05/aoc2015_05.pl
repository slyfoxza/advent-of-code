#!/usr/bin/env perl6
my $n1 = 0, my $n2 = 0;
for $*IN.lines {
	unless m/(ab|cd|pq|xy)/ {
		my $s = $_;
		$s ~~ s:g/<[b..z]-[aeiou]>//;
		$n1++ if m/(<:L>)$0/ && $s.chars >= 3
	}
	$n2++ if m/(<:L>**2)<:L>*$0/ && m/(<:L>)<:L>$0/
}
print "$n1 $n2\n"
# vim: set noet sw=4 ts=4:
